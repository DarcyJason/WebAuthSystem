pub mod request;
pub mod response;

use crate::application::app_state::AppState;
use crate::application::commands::reset_password_command::ResetPasswordCommand;
use crate::application::error::ValidationSnafu;
use crate::application::use_cases::reset_password_case::ResetPasswordCase;
use crate::domain::user::value_objects::credential::plain_password::PlainPassword;
use crate::presentation::http::v1::error::ApiResult;
use crate::presentation::http::v1::handlers::reset_password_handler::request::ResetPasswordRequestPayload;
use crate::presentation::http::v1::handlers::reset_password_handler::response::ResetPasswordResponseData;
use crate::presentation::http::v1::response::ApiResponse;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use std::sync::Arc;
use tracing::instrument;

#[utoipa::path(
    post,
    path = "/api/v1/auth/reset-password",
    tag = "Auth",
    request_body = ResetPasswordRequestPayload,
    responses(
        (status = 200, description = "Password reset successfully", body = ResetPasswordResponseData),
        (status = 400, description = "Token expired, already used, or passwords do not match"),
        (status = 404, description = "Token not found"),
    )
)]
#[instrument(skip(app_state, payload), fields(token_len=payload.token.len()))]
pub async fn reset_password_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<ResetPasswordRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    tracing::info!("handling reset password request");
    payload.validate_passwords()?;
    let new_password = PlainPassword::new(payload.new_password).map_err(|e| {
        ValidationSnafu {
            message: e.to_string(),
        }
        .build()
    })?;
    let case = ResetPasswordCase::new(
        app_state.user_repo.clone(),
        app_state.password_service.clone(),
        app_state.verification_token_repo.clone(),
    );
    case.execute(ResetPasswordCommand {
        token: payload.token,
        new_password,
    })
    .await?;
    let response = ApiResponse::<ResetPasswordResponseData>::ok(
        None,
        "Reset password successfully",
        ResetPasswordResponseData,
    );
    tracing::info!("handling reset password request successfully");
    Ok(response)
}
