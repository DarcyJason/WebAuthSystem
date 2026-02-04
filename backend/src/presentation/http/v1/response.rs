use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct EmptyResponseData;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    code: u16,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(code: u16, message: impl Into<String>, data: T) -> Self {
        ApiResponse {
            code,
            message: message.into(),
            data: Some(data),
        }
    }
    pub fn err(code: u16, message: impl Into<String>) -> Self {
        ApiResponse {
            code,
            message: message.into(),
            data: None,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = if self.code == 200 {
            axum::http::StatusCode::OK
        } else {
            axum::http::StatusCode::from_u16(self.code)
                .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        };
        (status, Json(self)).into_response()
    }
}
