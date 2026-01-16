pub mod redis_health;
pub mod surreal_health;

use crate::presentation::http::v1::{errors::AppResult, response::ApiResponse};
use axum::response::IntoResponse;

pub async fn health_handler() -> AppResult<impl IntoResponse> {
    let response = ApiResponse::<()>::ok(200, "Health", ());
    Ok(response)
}
