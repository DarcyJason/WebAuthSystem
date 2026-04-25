use crate::application::error::ApplicationError;
use crate::presentation::http::v1::response::{ApiResponse, EmptyResponseData};
use axum::response::IntoResponse;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    BadRequest { message: String },
    Unauthorized { message: String },
    NotFound { message: String },
    Conflict { message: String },
    InternalServerError { code: u32, message: String },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("handling it failed: {:?}", self);
        match self {
            ApiError::BadRequest { message } => {
                ApiResponse::<EmptyResponseData>::err(400, message).into_response()
            }
            ApiError::Unauthorized { message } => {
                ApiResponse::<EmptyResponseData>::err(401, message).into_response()
            }
            ApiError::NotFound { message } => {
                ApiResponse::<EmptyResponseData>::err(404, message).into_response()
            }
            ApiError::Conflict { message } => {
                ApiResponse::<EmptyResponseData>::err(409, message).into_response()
            }

            ApiError::InternalServerError { code, message } => {
                ApiResponse::<EmptyResponseData>::err(code, message).into_response()
            }
        }
    }
}

impl From<ApplicationError> for ApiError {
    fn from(error: ApplicationError) -> Self {
        match error {
            ApplicationError::RedisFailed { .. } => ApiError::InternalServerError {
                code: 500,
                message: "internal server error".to_string(),
            },
            ApplicationError::PostgresFailed { .. } => ApiError::InternalServerError {
                code: 500,
                message: "internal server error".to_string(),
            },
            ApplicationError::DomainFailed { .. } => ApiError::InternalServerError {
                code: 500,
                message: "internal server error".to_string(),
            },
            ApplicationError::PasswordServiceFailed { .. } => ApiError::InternalServerError {
                code: 500,
                message: "internal server error".to_string(),
            },
            ApplicationError::Validation { .. } => ApiError::BadRequest {
                message: error.to_string(),
            },
            ApplicationError::UserAlreadyExists { .. } => ApiError::Conflict {
                message: error.to_string(),
            },
            ApplicationError::UserNotFound { .. } => ApiError::NotFound {
                message: error.to_string(),
            },
            ApplicationError::InvalidCredentials { .. } => ApiError::BadRequest {
                message: error.to_string(),
            },
            ApplicationError::AccountIsBanned { .. } => ApiError::BadRequest {
                message: error.to_string(),
            },
            ApplicationError::EmailNotVerified { .. } => ApiError::BadRequest {
                message: error.to_string(),
            },
            ApplicationError::PasswordsNotMatched { .. } => ApiError::Conflict {
                message: error.to_string(),
            },
            ApplicationError::VerificationTokenNotFound { .. } => ApiError::NotFound {
                message: error.to_string(),
            },
            ApplicationError::VerificationTokenExpired { .. } => ApiError::BadRequest {
                message: error.to_string(),
            },
            ApplicationError::VerificationTokenAlreadyUsed { .. } => ApiError::BadRequest {
                message: error.to_string(),
            },
            ApplicationError::InvalidRefreshToken { .. } => ApiError::Unauthorized {
                message: error.to_string(),
            },
        }
    }
}
