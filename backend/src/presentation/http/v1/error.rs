use axum::response::IntoResponse;

use crate::presentation::http::v1::response::{ApiResponse, EmptyResponseData};

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
