use crate::{application::errors::AppError, presentation::http::v1::errors::api_error::ApiError};

impl From<AppError> for ApiError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::SurrealDBError => ApiError::internal_server_error(500, "SurrealDB error"),
            AppError::UserAlreadyExists => ApiError::conflict("User already exists"),
            AppError::CreateUserFailed => {
                ApiError::internal_server_error(500, "Failed to create user")
            }
            AppError::UserNotFound => ApiError::not_found("User not found"),
            AppError::HashPasswordFailed => {
                ApiError::internal_server_error(500, "Failed to hash password")
            }
            AppError::ParseHashedPasswordFailed => {
                ApiError::internal_server_error(500, "Failed to parse hashed password")
            }
            AppError::WrongIncredentials => ApiError::unauthorized("Invalid credentials"),
            AppError::EmailNotVerified => ApiError::unauthorized("Email not verified"),
            AppError::EncodeAccessTokenFailed => {
                ApiError::internal_server_error(500, "Failed to encode access_token")
            }
            AppError::DecodeAccessTokenFailed => {
                ApiError::internal_server_error(500, "Failed to decode access_token")
            }
            AppError::GenerateRefreshTokenFailed => {
                ApiError::internal_server_error(500, "Failed to generate refresh_token")
            }
            AppError::SystemOwnerEmailInvalid => {
                ApiError::internal_server_error(500, "System owner email invalid")
            }
            AppError::SendEmailVerificationFailed => {
                ApiError::internal_server_error(500, "Failed to send email")
            }
            AppError::SaveEmailVerificationTokenFailed => {
                ApiError::internal_server_error(500, "Failed to save email verification token")
            }
        }
    }
}
