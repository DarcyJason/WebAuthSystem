use chrono::Utc;
use ntex::http::HttpMessage;
use ntex::web::{
    HttpRequest, Responder,
    types::{Json, State},
};
use validator::ValidateEmail;

use crate::utils::token::{hash_token, validate_refresh_token};
use crate::{
    dtos::{
        api_response::ApiResponse,
        request::{login::LoginRequest, register::RegisterRequest},
        response::user::UserResponse,
    },
    errors::app_error::{AppError, AppResult},
    repositories::auth::AuthRepository,
    state::AppState,
    utils::{crypto::compare_hashed_password, token::generate_tokens},
};

pub async fn register_handler(
    app_state: State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<impl Responder> {
    if payload.name.is_empty() {
        return Err(AppError::NameEmpty);
    }
    if payload.email.is_empty() {
        return Err(AppError::EmailIsEmpty);
    }
    if !ValidateEmail::validate_email(&payload.email) {
        return Err(AppError::EmailIsInvalid);
    }
    if payload.password.is_empty() {
        return Err(AppError::PasswordEmpty);
    }
    if payload.confirm_password.is_empty() {
        return Err(AppError::ConfirmationPasswordEmpty);
    }
    if payload.password.len() < 8 {
        return Err(AppError::PasswordIsTooShort);
    }
    if payload.password.len() > 64 {
        return Err(AppError::PasswordIsTooLong);
    }
    if payload.confirm_password.len() < 8 {
        return Err(AppError::ConfirmationPasswordIsTooShort);
    }
    if payload.confirm_password.len() > 64 {
        return Err(AppError::ConfirmationPasswordIsTooLong);
    }
    if payload.password != payload.confirm_password {
        return Err(AppError::PasswordAndConfirmationPasswordAreNotMatched);
    }
    if let Some(_) = app_state
        .db_client
        .find_user_by_email(payload.email.clone())
        .await?
    {
        return Err(AppError::EmailAlreadyExists);
    }
    let user = app_state
        .db_client
        .create_user(payload.name, payload.email, payload.password)
        .await?;
    Ok(ApiResponse::success(
        "Register success",
        UserResponse::from(user),
    ))
}

pub async fn login_handler(
    app_state: State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<impl Responder> {
    if payload.email.is_empty() {
        return Err(AppError::EmailIsEmpty);
    }
    if !ValidateEmail::validate_email(&payload.email) {
        return Err(AppError::EmailIsInvalid);
    }
    if payload.password.is_empty() {
        return Err(AppError::PasswordEmpty);
    }
    if payload.password.len() < 8 {
        return Err(AppError::PasswordIsTooShort);
    }
    if payload.password.len() > 64 {
        return Err(AppError::PasswordIsTooLong);
    }
    let user = app_state
        .db_client
        .find_user_by_email(payload.email.clone())
        .await?
        .ok_or(AppError::UserNotFound)?;

    let is_password_matched = match compare_hashed_password(payload.password, user.password) {
        Ok(is_matched) => is_matched,
        Err(_) => return Err(AppError::InvalidCredentials),
    };

    if is_password_matched {
        match generate_tokens(
            user.id.clone(),
            app_state.config.token.jwt_secret_key.as_bytes(),
            app_state.config.token.access_expires_in_seconds,
            app_state.config.token.refresh_expires_in_seconds,
        ) {
            Ok((access_token, refresh_token)) => {
                let refresh_token_hash = hash_token(&refresh_token);
                let expires_at = Utc::now()
                    + chrono::Duration::seconds(
                        app_state.config.token.refresh_expires_in_seconds as i64,
                    );
                match app_state
                    .db_client
                    .store_refresh_token(user.id.clone(), refresh_token_hash, expires_at)
                    .await
                {
                    Ok(_) => {
                        let response = ApiResponse::success("Login successful", ())
                            .with_tokens(&access_token, Some(&refresh_token));
                        Ok(response)
                    }
                    Err(_) => Err(AppError::StoreRefreshTokenError),
                }
            }
            Err(_) => Err(AppError::GenerateTokenError),
        }
    } else {
        Err(AppError::InvalidCredentials)
    }
}

pub async fn logout_handler(
    req: HttpRequest,
    app_state: State<AppState>,
) -> AppResult<impl Responder> {
    if let Some(cookie) = req.cookie("refresh_token") {
        let raw_refresh_token = cookie.value().to_string();
        if let Ok(user_id) = validate_refresh_token(
            raw_refresh_token.clone(),
            app_state.config.token.jwt_secret_key.as_bytes(),
        ) {
            let token_hash = hash_token(&raw_refresh_token);
            if let Ok(Some(token)) = app_state
                .db_client
                .find_refresh_token(user_id, token_hash)
                .await
            {
                let _ = app_state
                    .db_client
                    .delete_refresh_token(token.id.unwrap())
                    .await;
            }
        }
    }
    Ok(ApiResponse::success("Logout successful", ()).revoke_tokens())
}

pub async fn refresh_handler(
    req: HttpRequest,
    app_state: State<AppState>,
) -> AppResult<impl Responder> {
    let refresh_token_cookie = req.cookie("refresh_token").ok_or(AppError::InvalidToken)?;
    let raw_refresh_token = refresh_token_cookie.value().to_string();
    let user_id = validate_refresh_token(
        raw_refresh_token.clone(),
        app_state.config.token.jwt_secret_key.as_bytes(),
    )?;
    let token_hash = hash_token(&raw_refresh_token);
    let stored_token = match app_state
        .db_client
        .find_refresh_token(user_id.clone(), token_hash)
        .await
    {
        Ok(Some(token)) => token,
        Ok(None) => return Err(AppError::InvalidToken),
        Err(_) => return Err(AppError::RefreshTokenNotFound),
    };
    match app_state
        .db_client
        .delete_refresh_token(stored_token.id.unwrap())
        .await
    {
        Ok(_) => (),
        Err(_) => return Err(AppError::DeleteRefreshTokenError),
    };
    match generate_tokens(
        user_id.clone(),
        app_state.config.token.jwt_secret_key.as_bytes(),
        app_state.config.token.access_expires_in_seconds,
        app_state.config.token.refresh_expires_in_seconds,
    ) {
        Ok((new_access_token, new_refresh_token)) => {
            let new_refresh_token_hash = hash_token(&new_refresh_token);
            let expires_at = Utc::now()
                + chrono::Duration::seconds(
                    app_state.config.token.refresh_expires_in_seconds as i64,
                );
            match app_state
                .db_client
                .store_refresh_token(user_id, new_refresh_token_hash, expires_at)
                .await
            {
                Ok(_) => {
                    let response = ApiResponse::success("Token refreshed successfully", ())
                        .with_tokens(&new_access_token, Some(&new_refresh_token));
                    Ok(response)
                }
                Err(_) => Err(AppError::StoreRefreshTokenError),
            }
        }
        Err(_) => Err(AppError::RefreshTokenError),
    }
}
