use crate::presentation::http::v1::errors::ApiResult;
use crate::presentation::http::v1::handlers::auth::reset_password_handler::request::ResetPasswordRequestPayload;
use crate::presentation::http::v1::states::app_state::AppState;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn reset_password_handler(
    Extension(_app_state): Extension<Arc<AppState>>,
    Json(_payload): Json<ResetPasswordRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    Ok("ok")
}
