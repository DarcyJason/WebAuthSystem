pub mod surreal_health;
pub mod redis_health;

use crate::presentation::http::v1::response::ApiResponse;
use crate::presentation::http::v1::result::AppResult;
use axum::response::IntoResponse;

pub async fn health_handler() -> AppResult<impl IntoResponse> {
    let response = ApiResponse::<()>::ok(200, "Health", ());
    Ok(response)
}
