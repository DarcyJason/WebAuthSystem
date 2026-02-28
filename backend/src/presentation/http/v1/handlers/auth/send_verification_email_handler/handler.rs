use axum::{Json, extract::State, response::IntoResponse};

use crate::{
    application::{
        commands::auth::send_verification_email_command::SendVerificationEmailCommand,
        use_cases::auth::send_verification_email_case::SendVerificationEmailCase,
    },
    presentation::http::v1::{
        errors::api_error::ApiResult,
        handlers::auth::send_verification_email_handler::{
            request::SendVerificationEmailRequestPayload,
            response::SendVerificationEmailResponseData,
        },
        response::ApiResponse,
        states::app_state::AppState,
    },
};

pub async fn send_verification_email_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<SendVerificationEmailRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    let cmd = SendVerificationEmailCommand::try_from(payload)?;
    let case = SendVerificationEmailCase::new(
        app_state.auth_mail_service.clone(),
        app_state.email_verification_cache.clone(),
    );
    let result = case.execute(cmd).await?;
    let response_data = SendVerificationEmailResponseData::from(result);
    let response = ApiResponse::<SendVerificationEmailResponseData>::ok(
        200,
        "Send verification email successfully",
        response_data,
    );
    Ok(response)
}
