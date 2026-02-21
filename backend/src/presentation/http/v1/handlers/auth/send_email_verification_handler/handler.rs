use axum::{Json, extract::State, response::IntoResponse};

use crate::{
    application::{
        commands::auth::send_email_verification_command::SendEmailVerificationCommand,
        use_cases::auth::send_email_verification_case::SendEmailVerificationCase,
    },
    presentation::http::v1::{
        errors::api_error::ApiResult,
        handlers::auth::send_email_verification_handler::{
            request::SendEmailVerificationRequestPayload,
            response::SendEmailVerificationResponseData,
        },
        response::ApiResponse,
        states::app_state::AppState,
    },
};

pub async fn send_email_verification_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<SendEmailVerificationRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    let cmd = SendEmailVerificationCommand::try_from(payload)?;
    let case = SendEmailVerificationCase::new(
        app_state.auth_mail_service.clone(),
        app_state.email_verification_cache.clone(),
    );
    let result = case.execute(cmd).await?;
    let response_data = SendEmailVerificationResponseData::from(result);
    let response = ApiResponse::<SendEmailVerificationResponseData>::ok(
        200,
        "Send email verification successfully",
        response_data,
    );
    Ok(response)
}
