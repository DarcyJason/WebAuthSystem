pub mod request;
pub mod response;

use crate::application::app_state::AppState;
use crate::application::use_cases::register_case::RegisterCase;
use crate::presentation::http::v1::error::ApiResult;
use crate::presentation::http::v1::handlers::register_handler::request::RegisterRequestPayload;
use crate::presentation::http::v1::handlers::register_handler::response::RegisterResponseData;
use crate::presentation::http::v1::response::ApiResponse;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use std::sync::Arc;
use tracing::instrument;

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "Auth",
    request_body = RegisterRequestPayload,
    responses(
        (status = 200, description = "Registered successfully, verification email sent", body = RegisterResponseData),
        (status = 400, description = "Validation error"),
        (status = 409, description = "User already exists"),
    )
)]
#[instrument(skip(app_state, payload), fields(name=%payload.name, email=%payload.email))]
pub async fn register_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    tracing::info!("handling register request");
    let cmd = payload.try_into()?;
    let case = RegisterCase::new(
        app_state.user_repo.clone(),
        app_state.password_service.clone(),
        app_state.verification_token_repo.clone(),
        app_state.verification_token_service.clone(),
        app_state.mail_service.clone(),
    );
    let result = case.execute(cmd).await?;
    let response_data = RegisterResponseData::from(&result);
    let response =
        ApiResponse::<RegisterResponseData>::ok(None, "registered successfully", response_data);
    tracing::info!("handling register request successfully");
    Ok(response)
}
