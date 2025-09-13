use ntex::web::{
    Responder,
    types::{Json, State},
};
use validator::ValidateEmail;

use crate::{
    dtos::{
        api_response::ApiResponse, request::register::RegisterRequest, response::user::UserResponse,
    },
    errors::app_error::{AppError, AppResult},
    repositories::auth::AuthRepository,
    state::AppState,
};

pub async fn register_handler(
    app_state: State<AppState>,
    payload: Json<RegisterRequest>,
) -> AppResult<impl Responder> {
    let payload = payload.into_inner();
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
