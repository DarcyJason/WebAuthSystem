pub mod request;
pub mod response;

use crate::application::app_state::AppState;
use crate::application::use_cases::resend_verification_case::ResendVerificationCase;
use crate::presentation::http::v1::error::ApiResult;
use crate::presentation::http::v1::handlers::resend_verification_handler::request::ResendVerificationRequestPayload;
use crate::presentation::http::v1::handlers::resend_verification_handler::response::ResendVerificationResponseData;
use crate::presentation::http::v1::response::ApiResponse;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use std::sync::Arc;
use tracing::instrument;

#[utoipa::path(
    post,
    path = "/api/v1/auth/resend-verification",
    tag = "Auth",
    request_body = ResendVerificationRequestPayload,
    responses(
        (status = 200, description = "Verification email sent if user exists and is not verified", body = ResendVerificationResponseData),
        (status = 400, description = "Validation error"),
    )
)]
#[instrument(skip(app_state, payload), fields(email=%payload.email))]
pub async fn resend_verification_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<ResendVerificationRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    tracing::info!("handling resend verification request");
    let cmd = payload.try_into()?;
    let case = ResendVerificationCase::new(
        app_state.user_repo.clone(),
        app_state.verification_token_repo.clone(),
        app_state.verification_token_service.clone(),
        app_state.mail_service.clone(),
    );
    case.execute(cmd).await?;
    let response = ApiResponse::<ResendVerificationResponseData>::ok(
        None,
        "Verification email sent if user exists and is not verified",
        ResendVerificationResponseData,
    );
    tracing::info!("handling resend verification request successfully");
    Ok(response)
}
