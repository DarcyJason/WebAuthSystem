pub mod request;
pub mod response;

use crate::application::app_state::AppState;
use crate::presentation::http::v1::error::ApiResult;
use crate::presentation::http::v1::handlers::register_handler::request::RegisterRequest;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn register_handler(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> ApiResult<impl IntoResponse> {
    let cmd = req.into_command()?;
    Ok("handler/register_handler".to_string())
}
