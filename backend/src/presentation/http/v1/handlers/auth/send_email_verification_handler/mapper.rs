use thiserror::Error;

use crate::{
    application::{
        commands::auth::send_email_verification_command::SendEmailVerificationCommand,
        results::commands_results::auth::send_email_verification_result::SendEmailVerificationResult,
    },
    domain::auth::value_objects::user_email::{UserEmail, UserEmailError},
    presentation::http::v1::{
        errors::api_error::ApiError,
        handlers::auth::send_email_verification_handler::{
            request::SendEmailVerificationRequestPayload,
            response::SendEmailVerificationResponseData,
        },
    },
};

#[derive(Debug, Error)]
pub enum SendEmailVerificationRequestPayloadError {
    #[error("email is required")]
    UserEmailRequired,
    #[error("email is invalid")]
    UserEmailInvalid,
}

impl From<SendEmailVerificationRequestPayloadError> for ApiError {
    fn from(err: SendEmailVerificationRequestPayloadError) -> Self {
        ApiError::BadRequest {
            message: err.to_string(),
        }
    }
}

impl TryFrom<SendEmailVerificationRequestPayload> for SendEmailVerificationCommand {
    type Error = SendEmailVerificationRequestPayloadError;
    fn try_from(payload: SendEmailVerificationRequestPayload) -> Result<Self, Self::Error> {
        let email = UserEmail::new(payload.email).map_err(|e| match e {
            UserEmailError::UserEmailRequired => {
                SendEmailVerificationRequestPayloadError::UserEmailRequired
            }
            UserEmailError::UserEmailInvalid => {
                SendEmailVerificationRequestPayloadError::UserEmailInvalid
            }
        })?;
        Ok(SendEmailVerificationCommand { email })
    }
}

impl From<SendEmailVerificationResult> for SendEmailVerificationResponseData {
    fn from(_result: SendEmailVerificationResult) -> Self {
        SendEmailVerificationResponseData {}
    }
}
