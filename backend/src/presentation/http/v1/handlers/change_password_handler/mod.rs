pub mod request;
pub mod response;

use crate::application::app_state::AppState;
use crate::application::commands::change_password_command::ChangePasswordCommand;
use crate::application::use_cases::change_password_case::ChangePasswordCase;
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
    path = "/api/v1/protected/change-password",
    tag = "User",
    security(("Bearer" = [])),
    request_body = ChangePasswordRequestPayload,
    responses(
        (status = 200, description = "Password changed successfully", body = ChangePasswordResponseData),
        (status = 400, description = "Invalid current password or passwords do not match"),
        (status = 401, description = "Unauthorized"),
    )
)]
#[instrument(skip(app_state, auth, payload), fields(user_id=%auth.user.id()))]
pub async fn change_password_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthMiddleware>,
    Json(payload): Json<ChangePasswordRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    tracing::info!("handling change password request");
    let cmd: ChangePasswordCommand = payload.try_into()?;
    let user_id = auth.user.id();
    let case = ChangePasswordCase::new(
        user_id.clone(),
        app_state.user_repo.clone(),
        app_state.password_service.clone(),
    );
    let result = case.execute(cmd).await?;
    let response_data = ChangePasswordResponseData::from(result);
    let response = ApiResponse::<ChangePasswordResponseData>::ok(
        None,
        "Change password successfully",
        response_data,
    );
    tracing::info!("handling change password request successfully");
    Ok(response)
}
