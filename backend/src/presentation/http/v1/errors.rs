use crate::{application::errors::ApplicationError, presentation::http::v1::response::ApiResponse};
use axum::response::IntoResponse;
use tracing::error;

pub type ApiResult<T> = Result<T, ApiError>;

pub enum ApiError {
    BadRequest { message: String },
    NotFound { message: String },
    Conflict { message: String },
    Unauthorized,
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        error!("Handle it Failed");
        match self {
            ApiError::BadRequest { message } => {
                ApiResponse::<()>::err(400, message).into_response()
            }
            ApiError::NotFound { message } => ApiResponse::<()>::err(404, message).into_response(),
            ApiError::Conflict { message } => ApiResponse::<()>::err(409, message).into_response(),
            ApiError::Unauthorized => {
                ApiResponse::<()>::err(401, "invalid credentials").into_response()
            }
            ApiError::InternalServerError => {
                ApiResponse::<()>::err(500, "internal server error").into_response()
            }
        }
    }
}

impl From<ApplicationError> for ApiError {
    fn from(err: ApplicationError) -> Self {
        match err {
            ApplicationError::InvalidCredentials => ApiError::Unauthorized,
            ApplicationError::UsernotFound => ApiError::NotFound {
                message: err.to_string(),
            },
            ApplicationError::UserAlreadyExists => ApiError::Conflict {
                message: err.to_string(),
            },
            _ => ApiError::InternalServerError,
        }
    }
}
