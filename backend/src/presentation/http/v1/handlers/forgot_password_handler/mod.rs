pub mod request;
pub mod response;

use crate::application::app_state::AppState;
use crate::application::commands::forgot_password_command::ForgotPasswordCommand;
use crate::application::use_cases::forgot_password_case::ForgotPasswordCase;
use crate::presentation::http::v1::error::ApiResult;
use crate::presentation::http::v1::handlers::forgot_password_handler::request::ForgotPasswordRequestPayload;
use crate::presentation::http::v1::handlers::forgot_password_handler::response::ForgotPasswordResponseData;
use crate::presentation::http::v1::response::ApiResponse;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use std::sync::Arc;
use tracing::instrument;

#[utoipa::path(
    post,
    path = "/api/v1/auth/forgot-password",
    tag = "Auth",
    request_body = ForgotPasswordRequestPayload,
    responses(
        (status = 200, description = "If the email is registered, a password reset email has been sent", body = ForgotPasswordResponseData),
        (status = 400, description = "Validation error"),
    )
)]
#[instrument(skip(app_state, payload), fields(email=%payload.email))]
pub async fn forgot_password_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<ForgotPasswordRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    tracing::info!("handling forgot password request");
    let email = payload.try_into()?;
    let case = ForgotPasswordCase::new(
        app_state.user_repo.clone(),
        app_state.verification_token_repo.clone(),
        app_state.verification_token_service.clone(),
        app_state.mail_service.clone(),
    );
    case.execute(ForgotPasswordCommand { email }).await?;
    let response = ApiResponse::<ForgotPasswordResponseData>::ok(
        None,
        "If this email is registered, a password reset email has been sent",
        ForgotPasswordResponseData,
    );
    tracing::info!("handling forgot password request successfully");
    Ok(response)
}
