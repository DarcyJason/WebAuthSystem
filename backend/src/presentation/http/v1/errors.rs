use crate::application::errors::CaseError;
use crate::presentation::http::v1::response::{ApiResponse, EmptyResponseData};
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Serialize)]
pub enum ApiError {
    BadRequest { message: String },
    Unauthorized { message: String },
    NotFound { message: String },
    Conflict { message: String },
    InternalServerError { code: u16, message: String },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
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

impl From<CaseError> for ApiError {
    fn from(e: CaseError) -> Self {
        match e {
            CaseError::CredentialsInvalid | CaseError::EmailVerificationTokenInvalid => {
                ApiError::BadRequest {
                    message: e.to_string(),
                }
            }
            CaseError::EmailNotVerified => ApiError::Unauthorized {
                message: e.to_string(),
            },
            CaseError::UserNotFound | CaseError::EmailVerificationTokenNotFound => {
                ApiError::NotFound {
                    message: e.to_string(),
                }
            }
            CaseError::UserAlreadyExists => ApiError::Conflict {
                message: e.to_string(),
            },
            _ => ApiError::InternalServerError {
                code: e.clone().status_code(),
                message: e.to_string(),
            },
        }
    }
}
