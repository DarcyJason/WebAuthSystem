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
    #[error("Password is too long")]
    PasswordIsTooLong,
    #[error("Password is not matched")]
    PasswordIsNotMatched,
    #[error("Authentication failed")]
    AuthenticationError,
    #[error("User creation failed")]
    UserCreationError,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
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
            AppError::PasswordIsTooLong => StatusCode::BAD_REQUEST,
            AppError::PasswordIsNotMatched => StatusCode::BAD_REQUEST,
            AppError::AuthenticationError => StatusCode::UNAUTHORIZED,
            AppError::UserCreationError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UserAlreadyExists => StatusCode::CONFLICT,
            AppError::UserNotFound => StatusCode::NOT_FOUND,
            AppError::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::OtherError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
