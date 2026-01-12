use axum::response::IntoResponse;
use crate::presentation::http::v1::response::ApiResponse;
use crate::presentation::http::v1::result::AppResult;

pub async fn health_handler() -> AppResult<impl IntoResponse> {
    let response = ApiResponse::<()>::ok(200, "Health", ());
    Ok(response)
}