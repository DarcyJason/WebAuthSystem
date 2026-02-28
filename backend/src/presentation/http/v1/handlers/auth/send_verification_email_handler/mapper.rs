use thiserror::Error;

use crate::{
    application::{
        commands::auth::send_verification_email_command::SendVerificationEmailCommand,
        results::commands_results::auth::send_verification_email_result::SendVerificationEmailResult,
    },
    domain::user::value_objects::user_email::{UserEmail, UserEmailError},
    presentation::http::v1::{
        errors::api_error::ApiError,
        handlers::auth::send_verification_email_handler::{
            request::SendVerificationEmailRequestPayload,
            response::SendVerificationEmailResponseData,
        },
    },
};

#[derive(Debug, Error)]
pub enum SendVerificationEmailRequestPayloadError {
    #[error("email is required")]
    UserEmailRequired,
    #[error("email is invalid")]
    UserEmailInvalid,
}

impl From<SendVerificationEmailRequestPayloadError> for ApiError {
    fn from(err: SendVerificationEmailRequestPayloadError) -> Self {
        ApiError::BadRequest {
            message: err.to_string(),
        }
    }
}

impl TryFrom<SendVerificationEmailRequestPayload> for SendVerificationEmailCommand {
    type Error = SendVerificationEmailRequestPayloadError;
    fn try_from(payload: SendVerificationEmailRequestPayload) -> Result<Self, Self::Error> {
        let email = UserEmail::new(payload.email).map_err(|e| match e {
            UserEmailError::UserEmailRequired => {
                SendVerificationEmailRequestPayloadError::UserEmailRequired
            }
            UserEmailError::UserEmailInvalid => {
                SendVerificationEmailRequestPayloadError::UserEmailInvalid
            }
        })?;
        Ok(SendVerificationEmailCommand { email })
    }
}

impl From<SendVerificationEmailResult> for SendVerificationEmailResponseData {
    fn from(_result: SendVerificationEmailResult) -> Self {
        SendVerificationEmailResponseData {}
    }
}
