use crate::application::auth::commands::forgot_password_command::ForgotPasswordCommand;
use crate::presentation::http::v1::errors::ApiResult;
use crate::presentation::http::v1::handlers::auth::forgot_password_handler::request::ForgotPasswordRequestPayload;
use crate::presentation::http::v1::states::AppState;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn forgot_password_handler(
    Extension(_app_state): Extension<Arc<AppState>>,
    Json(payload): Json<ForgotPasswordRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    let _cmd = ForgotPasswordCommand::try_from(payload)?;
    Ok("ok")
}
