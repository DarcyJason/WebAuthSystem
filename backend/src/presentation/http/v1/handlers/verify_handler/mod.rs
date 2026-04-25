pub mod request;
pub mod response;

use crate::application::app_state::AppState;
use crate::application::use_cases::verify_case::VerifyCase;
use crate::presentation::http::v1::error::ApiResult;
use crate::presentation::http::v1::handlers::verify_handler::request::VerifyRequestPayload;
use crate::presentation::http::v1::handlers::verify_handler::response::VerifyResponseData;
use crate::presentation::http::v1::response::ApiResponse;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use std::sync::Arc;
use tracing::instrument;

#[utoipa::path(
    post,
    path = "/api/v1/auth/verify",
    tag = "Auth",
    request_body = VerifyRequestPayload,
    responses(
        (status = 200, description = "Email verified, account activated", body = VerifyResponseData),
        (status = 400, description = "Token expired or already used"),
        (status = 404, description = "Token not found"),
    )
)]
#[instrument(skip(app_state, payload), fields(token_len=payload.token.len()))]
pub async fn verify_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<VerifyRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    tracing::info!("handling verify request");
    let cmd = payload.try_into()?;
    let case = VerifyCase::new(
        app_state.verification_token_repo.clone(),
        app_state.user_repo.clone(),
    );
    let result = case.execute(cmd).await?;
    let response_data = VerifyResponseData::from(result);
    let response =
        ApiResponse::<VerifyResponseData>::ok(None, "Email verified successfully", response_data);
    tracing::info!("handling verify request successfully");
    Ok(response)
}
