use axum::{Json, extract::State, response::IntoResponse};

use crate::{
    app_state::AppState,
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
    },
};

pub async fn send_email_verification_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<SendEmailVerificationRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    let send_email_verification_command = SendEmailVerificationCommand::try_from(payload)?;
    let case = SendEmailVerificationCase::new(
        app_state.auth_mail_service.clone(),
        app_state.email_verification_cache.clone(),
    );
    let send_email_verification_result = case.execute(send_email_verification_command).await?;
    let response_data = SendEmailVerificationResponseData::from(send_email_verification_result);
    let response = ApiResponse::<SendEmailVerificationResponseData>::ok(
        200,
        "Send email verification successfully",
        response_data,
    );
    Ok(response)
}
