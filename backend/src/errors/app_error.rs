use ntex::{
    http::StatusCode,
    web::{self, WebResponseError},
};
use thiserror::Error;

use crate::dtos::api_response::ApiResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(Box<figment::Error>),
    #[error("SurrealDB error: {0}")]
    SurrealDBError(Box<surrealdb::Error>),
    #[error("Password hashing error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),
    #[error("Password is empty")]
    PasswordEmpty,
    #[error("Confirmation password is empty")]
    ConfirmationPasswordEmpty,
    #[error("Password is too short, at least 8 characters")]
    PasswordIsTooShort,
    #[error("Password is too long, at most 64 characters")]
    PasswordIsTooLong,
    #[error("Confirmation password is too short, at least 8 characters")]
    ConfirmationPasswordIsTooShort,
    #[error("Confirmation password is too long, at most 64 characters")]
    ConfirmationPasswordIsTooLong,
    #[error("Password and confirmation password are not matched")]
    PasswordAndConfirmationPasswordAreNotMatched,
    #[error("Name is empty")]
    NameEmpty,
    #[error("Email is empty")]
    EmailIsEmpty,
    #[error("Email is invalid")]
    EmailIsInvalid,
    #[error("Email already exists")]
    EmailAlreadyExists,
    #[error("Authentication failed")]
    AuthenticationError,
    #[error("User creation failed")]
    UserCreationError,
    #[error("User not found")]
    UserNotFound,
    #[error("Generate token failed")]
    GenerateTokenError,
    #[error("Refresh token failed")]
    RefreshTokenError,
    #[error("Store refresh tokens failed")]
    StoreRefreshTokenError,
    #[error("Delete refresh tokens failed")]
    DeleteRefreshTokenError,
    #[error("Refresh token not found")]
    RefreshTokenNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token error")]
    TokenError(#[from] jsonwebtoken::errors::Error),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Other error: {0}")]
    OtherError(Box<anyhow::Error>),
}

impl From<figment::Error> for AppError {
    fn from(err: figment::Error) -> Self {
        AppError::ConfigError(Box::new(err))
    }
}

impl From<surrealdb::Error> for AppError {
    fn from(err: surrealdb::Error) -> Self {
        AppError::SurrealDBError(Box::new(err))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::OtherError(Box::new(err))
    }
}

impl WebResponseError for AppError {
    fn error_response(&self, _: &ntex::web::HttpRequest) -> ntex::http::Response {
        let api_response: ApiResponse<()> =
            ApiResponse::error(self.status_code(), self.to_string().as_str());
        web::HttpResponse::build(self.status_code())
            .content_type("application/json")
            .body(serde_json::to_string(&api_response).unwrap())
    }
    fn status_code(&self) -> ntex::http::StatusCode {
        match self {
            AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SurrealDBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordHashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordEmpty => StatusCode::BAD_REQUEST,
            AppError::ConfirmationPasswordEmpty => StatusCode::BAD_REQUEST,
            AppError::PasswordIsTooShort => StatusCode::BAD_REQUEST,
            AppError::PasswordIsTooLong => StatusCode::BAD_REQUEST,
            AppError::ConfirmationPasswordIsTooShort => StatusCode::BAD_REQUEST,
            AppError::ConfirmationPasswordIsTooLong => StatusCode::BAD_REQUEST,
            AppError::PasswordAndConfirmationPasswordAreNotMatched => StatusCode::BAD_REQUEST,
            AppError::NameEmpty => StatusCode::BAD_REQUEST,
            AppError::EmailIsInvalid => StatusCode::BAD_REQUEST,
            AppError::EmailIsEmpty => StatusCode::BAD_REQUEST,
            AppError::EmailAlreadyExists => StatusCode::CONFLICT,
            AppError::AuthenticationError => StatusCode::UNAUTHORIZED,
            AppError::UserCreationError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UserNotFound => StatusCode::NOT_FOUND,
            AppError::GenerateTokenError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::RefreshTokenError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::StoreRefreshTokenError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DeleteRefreshTokenError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::RefreshTokenNotFound => StatusCode::NOT_FOUND,
            AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AppError::InvalidToken => StatusCode::UNAUTHORIZED,
            AppError::TokenError(_) => StatusCode::UNAUTHORIZED,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::OtherError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
