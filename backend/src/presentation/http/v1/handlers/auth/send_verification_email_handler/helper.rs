use thiserror::Error;

use crate::domain::user::entities::user::user_email::{UserEmail, UserEmailError};
use crate::presentation::http::v1::errors::ApiError;
use crate::{
    application::{
        commands::auth::send_verification_email_command::SendVerificationEmailCommand,
        results::commands_results::auth::send_verification_email_result::SendVerificationEmailResult,
    },
    presentation::http::v1::handlers::auth::send_verification_email_handler::{
        request::SendVerificationEmailRequestPayload, response::SendVerificationEmailResponseData,
    },
};

#[derive(Debug, Error)]
pub enum SendVerificationEmailRequestPayloadError {
    #[error("email is required")]
    UserEmailRequired,
    #[error("email is invalid")]
    UserEmailInvalid,
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

impl From<SendVerificationEmailRequestPayloadError> for ApiError {
    fn from(e: SendVerificationEmailRequestPayloadError) -> Self {
        ApiError::BadRequest {
            message: e.to_string(),
        }
    }
}

impl From<SendVerificationEmailResult> for SendVerificationEmailResponseData {
    fn from(_result: SendVerificationEmailResult) -> Self {
        SendVerificationEmailResponseData {}
    }
}
