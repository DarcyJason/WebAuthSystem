use crate::presentation::http::v1::response::{ApiResponse, EmptyResponseData};
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Serialize)]
pub enum ApiError {
    BadRequest { message: String },
    Unauthorized { message: String },
    Forbidden { message: String },
    NotFound { message: String },
    RequestTimeout { message: String },
    Conflict { message: String },
    InternalServerError { code: u16, message: String },
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest {
            message: message.into(),
        }
    }
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::Unauthorized {
            message: message.into(),
        }
    }
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::Forbidden {
            message: message.into(),
        }
    }
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound {
            message: message.into(),
        }
    }
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict {
            message: message.into(),
        }
    }
    pub fn internal_server_error(code: u16, message: impl Into<String>) -> Self {
        Self::InternalServerError {
            code,
            message: message.into(),
        }
    }
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
            ApiError::Forbidden { message } => {
                ApiResponse::<EmptyResponseData>::err(403, message).into_response()
            }
            ApiError::NotFound { message } => {
                ApiResponse::<EmptyResponseData>::err(404, message).into_response()
            }
            ApiError::RequestTimeout { message } => {
                ApiResponse::<EmptyResponseData>::err(408, message).into_response()
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
