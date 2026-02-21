use axum::{Json, extract::State, response::IntoResponse};

use crate::{
    application::{
        commands::auth::validate_email_verification_command::ValidateEmailVerificationCommand,
        use_cases::auth::validate_email_verification_case::ValidateEmailVerificationCase,
    },
    presentation::http::v1::{
        errors::api_error::ApiResult,
        handlers::auth::validate_email_verification_handler::{
            request::ValidateEmailVerificationRequestPayload,
            response::ValidateEmailVerificationResponseData,
        },
        response::ApiResponse,
        states::app_state::AppState,
    },
};

pub async fn validate_email_verification_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<ValidateEmailVerificationRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    let cmd = ValidateEmailVerificationCommand::try_from(payload)?;
    let case = ValidateEmailVerificationCase::new(app_state.email_verification_cache.clone());
    let result = case.execute(cmd).await?;
    let response_data = ValidateEmailVerificationResponseData::from(result);
    let response = ApiResponse::<ValidateEmailVerificationResponseData>::ok(
        200,
        "verify your email successfully",
        response_data,
    );
    Ok(response)
}
