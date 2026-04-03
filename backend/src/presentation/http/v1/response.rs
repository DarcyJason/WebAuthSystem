use axum::Json;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct EmptyResponseData;

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    #[serde(skip_serializing)]
    headers: Option<HeaderMap<HeaderValue>>, // Add AUTHORIZATION, SET_COOKIE here
    code: u32,
    message: String,
    data: Option<T>, // If it's none, use EmptyResponseData
}

impl<T> ApiResponse<T> {
    pub fn ok(
        headers: Option<HeaderMap<HeaderValue>>,
        message: impl Into<String>,
        data: T,
    ) -> Self {
        ApiResponse {
            headers,
            code: 200,
            message: message.into(),
            data: Some(data),
        }
    }
    pub fn err(code: u32, message: impl Into<String>) -> Self {
        ApiResponse {
            headers: None,
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
        let status = if self.code <= 65535 {
            StatusCode::from_u16(self.code as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };
        (status, self.headers.clone(), Json(self)).into_response()
    }
}
