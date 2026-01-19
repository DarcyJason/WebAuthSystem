use crate::{application::errors::ApplicationError, presentation::http::v1::response::ApiResponse};
use axum::response::IntoResponse;

pub type ApiResult<T> = Result<T, ApiError>;

pub enum ApiError {
    BadRequest { code: u16, message: String },
    Unauthorized,
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::BadRequest { code, message } => {
                ApiResponse::<()>::err(code, message).into_response()
            }
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
            _ => ApiError::InternalServerError,
        }
    }
}
