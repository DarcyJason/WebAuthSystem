use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::application::errors::ApplicationError;
use crate::presentation::http::v1::response::ApiResponse;

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response { 
        let (http_status, business_code, message) = match self {
           ApplicationError::Domain(_) => (
               StatusCode::INTERNAL_SERVER_ERROR,
               10000,
               self.to_string(),
           ),
           ApplicationError::RespoitoryUnavailable => (
               StatusCode::INTERNAL_SERVER_ERROR,
               20000,
               self.to_string()
           )
        };
        let response = ApiResponse::<()>::err(business_code, &message);
        (http_status, axum::Json(response)).into_response()
    }
}