use crate::application::error::ApplicationError;
use crate::presentation::http::v1::response::{ApiResponse, EmptyResponseData};
use axum::response::IntoResponse;

pub type ApiResult<T> = Result<T, ApiError>;

pub enum ApiError {
    BadRequest { message: String },
    Unauthorized { message: String },
    NotFound { message: String },
    Conflict { message: String },
    InternalServerError { code: u32, message: String },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
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
            ApplicationError::Redis { .. } => ApiError::InternalServerError {
                code: 500,
                message: "internal server error".to_string(),
            },
            ApplicationError::Postgres { .. } => ApiError::InternalServerError {
                code: 500,
                message: "internal server error".to_string(),
            },
            ApplicationError::FieldInvalid { .. } => ApiError::BadRequest {
                message: error.to_string(),
            },
        }
    }
}
