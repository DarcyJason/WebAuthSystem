use axum::{Extension, Json, response::IntoResponse};
use std::sync::Arc;

use crate::presentation::http::v1::errors::ApiResult;
use crate::{
    application::{
        commands::auth::validate_verification_command::ValidateVerificationCommand,
        use_cases::auth::validate_verification_case::ValidateVerificationCase,
    },
    presentation::http::v1::{
        handlers::auth::validate_verification_handler::{
            request::ValidateEmailVerificationRequestPayload,
            response::ValidateEmailVerificationResponseData,
        },
        response::ApiResponse,
        states::app_state::AppState,
    },
};

pub async fn validate_verification_handler(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(payload): Json<ValidateEmailVerificationRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    let cmd = ValidateVerificationCommand::try_from(payload)?;
    let case = ValidateVerificationCase::new(
        app_state.email_verification_cache.clone(),
        app_state.user_repo.clone(),
    );
    let result = case.execute(cmd).await?;
    let response_data = ValidateEmailVerificationResponseData::from(result);
    let response = ApiResponse::<ValidateEmailVerificationResponseData>::ok(
        200,
        "verify your email successfully",
        response_data,
    );
    Ok(response)
}
