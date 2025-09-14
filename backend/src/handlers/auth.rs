use ntex::web::{
    Responder,
    types::{Json, State},
};
use validator::ValidateEmail;

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

    let is_password_matched = compare_hashed_password(payload.password, user.password)?;

    if is_password_matched {
        let (access_token, refresh_token) = generate_tokens(
            user.id,
            app_state.config.token.jwt_secret_key.as_bytes(),
            app_state.config.token.access_expires_in_seconds,
            app_state.config.token.refresh_expires_in_seconds,
        )?;

        let response = ApiResponse::success("Login successful", ())
            .with_tokens(&access_token, Some(&refresh_token));

        Ok(response)
    } else {
        Err(AppError::InvalidCredentials)
    }
}

pub async fn logout_handler() -> AppResult<impl Responder> {
    Ok(ApiResponse::success("Logout successful", ()).revoke_tokens())
}
