use crate::presentation::http::v1::{errors::ApiResult, response::ApiResponse};
use axum::response::IntoResponse;
use tracing::{info, instrument};

#[instrument]
pub async fn backend_health_handler() -> ApiResult<impl IntoResponse> {
    info!("Start handling backend health");
    let response = ApiResponse::<()>::ok(200, "Healthy", ());
    info!("Finish handling backend health");
    Ok(response)
}
