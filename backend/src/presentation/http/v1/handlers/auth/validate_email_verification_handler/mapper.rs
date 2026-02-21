use thiserror::Error;

use crate::{
    application::{
        commands::auth::validate_email_verification_command::ValidateEmailVerificationCommand,
        results::commands_results::auth::validate_email_verification_result::ValidateEmailVerificationResult,
    },
    domain::auth::value_objects::{
        user_email::{UserEmail, UserEmailError},
        verification_token::VerificationToken,
    },
    presentation::http::v1::{
        errors::api_error::ApiError,
        handlers::auth::validate_email_verification_handler::{
            request::ValidateEmailVerificationRequestPayload,
            response::ValidateEmailVerificationResponseData,
        },
    },
};

#[derive(Debug, Error)]
pub enum ValidateEmailVerificationRequestPayloadError {
    #[error("email is required")]
    UserEmailRequired,
    #[error("email is invalid")]
    UserEmailInvalid,
}

impl From<ValidateEmailVerificationRequestPayloadError> for ApiError {
    fn from(err: ValidateEmailVerificationRequestPayloadError) -> Self {
        ApiError::BadRequest {
            message: err.to_string(),
        }
    }
}

impl TryFrom<ValidateEmailVerificationRequestPayload> for ValidateEmailVerificationCommand {
    type Error = ValidateEmailVerificationRequestPayloadError;
    fn try_from(payload: ValidateEmailVerificationRequestPayload) -> Result<Self, Self::Error> {
        let email = UserEmail::new(payload.email).map_err(|e| match e {
            UserEmailError::UserEmailRequired => {
                ValidateEmailVerificationRequestPayloadError::UserEmailRequired
            }
            UserEmailError::UserEmailInvalid => {
                ValidateEmailVerificationRequestPayloadError::UserEmailInvalid
            }
        })?;
        let verification_token = VerificationToken::from(payload.verification_token);
        Ok(ValidateEmailVerificationCommand {
            email,
            verification_token,
        })
    }
}

impl From<ValidateEmailVerificationResult> for ValidateEmailVerificationResponseData {
    fn from(_result: ValidateEmailVerificationResult) -> Self {
        ValidateEmailVerificationResponseData {}
    }
}
