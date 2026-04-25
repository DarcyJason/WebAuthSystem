pub mod request;
pub mod response;

use crate::application::app_state::AppState;
use crate::presentation::http::v1::error::ApiResult;
use crate::presentation::http::v1::handlers::change_password_handler::request::ChangePasswordRequestPayload;
use crate::presentation::http::v1::handlers::change_password_handler::response::ChangePasswordResponseData;
use crate::presentation::http::v1::middlewares::auth::AuthMiddleware;
use crate::presentation::http::v1::response::ApiResponse;
use axum::Json;
use axum::extract::{Extension, State};
use axum::response::IntoResponse;
use std::sync::Arc;
use tracing::instrument;

#[utoipa::path(
    post,
    path = "/api/v1/change-password",
    tag = "User",
    security(("bearer_auth" = [])),
    request_body = ChangePasswordRequestPayload,
    responses(
        (status = 200, description = "Password changed successfully", body = ChangePasswordResponseData),
        (status = 400, description = "Invalid current password or passwords do not match"),
        (status = 401, description = "Unauthorized"),
    )
)]
#[instrument(skip(_app_state, auth, payload), fields(user_id=%auth.user.id()))]
pub async fn change_password_handler(
    State(_app_state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthMiddleware>,
    Json(payload): Json<ChangePasswordRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    tracing::info!("handling change password request");
    payload.validate_passwords()?;
    let _user_id = auth.user.id();
    // TODO: verify current password and update password credential.
    let response = ApiResponse::<ChangePasswordResponseData>::ok(
        None,
        "Change password successfully",
        ChangePasswordResponseData,
    );
    tracing::info!("handling change password request successfully");
    Ok(response)
}
