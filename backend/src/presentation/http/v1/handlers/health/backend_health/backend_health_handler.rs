use crate::presentation::http::v1::{errors::ApiResult, response::ApiResponse};
use axum::response::IntoResponse;
use tracing::instrument;

#[instrument]
pub async fn health_handler() -> ApiResult<impl IntoResponse> {
    let response = ApiResponse::<()>::ok(200, "Healthy", ());
    Ok(response)
}
