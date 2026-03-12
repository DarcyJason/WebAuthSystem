use axum::{response::IntoResponse, Extension, Json};
use std::sync::Arc;

use crate::presentation::http::v1::errors::ApiResult;
use crate::presentation::http::v1::states::AppState;
use crate::{
    application::{
        commands::auth::send_verification_email_command::SendVerificationEmailCommand,
        use_cases::auth::send_verification_email_case::SendVerificationEmailCase,
    },
    presentation::http::v1::{
        handlers::auth::send_verification_email_handler::{
            request::SendVerificationEmailRequestPayload,
            response::SendVerificationEmailResponseData,
        },
        response::ApiResponse,
    },
};

pub async fn send_verification_email_handler(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(payload): Json<SendVerificationEmailRequestPayload>,
) -> ApiResult<impl IntoResponse> {
    let cmd = SendVerificationEmailCommand::try_from(payload)?;
    let case = SendVerificationEmailCase::new(
        app_state.mail_service.clone(),
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
