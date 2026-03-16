use thiserror::Error;

use crate::application::auth::commands::validate_verification_command::ValidateVerificationCommand;
use crate::application::auth::results::validate_verification_result::ValidateVerificationResult;
use crate::domain::auth::errors::user::user_email_error::UserEmailError;
use crate::domain::auth::value_objects::tokens::verification_token::VerificationToken;
use crate::domain::auth::value_objects::user::user_email::UserEmail;
use crate::presentation::http::v1::errors::ApiError;
use crate::presentation::http::v1::handlers::auth::validate_verification_handler::{
    request::ValidateEmailVerificationRequestPayload,
    response::ValidateEmailVerificationResponseData,
};

#[derive(Debug, Error)]
pub enum ValidateVerificationRequestPayloadError {
    #[error("email is required")]
    UserEmailRequired,
    #[error("email is invalid")]
    UserEmailInvalid,
}

impl TryFrom<ValidateEmailVerificationRequestPayload> for ValidateVerificationCommand {
    type Error = ValidateVerificationRequestPayloadError;
    fn try_from(payload: ValidateEmailVerificationRequestPayload) -> Result<Self, Self::Error> {
        let email = UserEmail::new(payload.email).map_err(|e| match e {
            UserEmailError::UserEmailRequired => {
                ValidateVerificationRequestPayloadError::UserEmailRequired
            }
            UserEmailError::UserEmailInvalid => {
                ValidateVerificationRequestPayloadError::UserEmailInvalid
            }
        })?;
        let verification_token = VerificationToken::from(payload.verification_token);
        Ok(ValidateVerificationCommand {
            email,
            verification_token,
        })
    }
}

impl From<ValidateVerificationRequestPayloadError> for ApiError {
    fn from(e: ValidateVerificationRequestPayloadError) -> Self {
        ApiError::BadRequest {
            message: e.to_string(),
        }
    }
}

impl From<ValidateVerificationResult> for ValidateEmailVerificationResponseData {
    fn from(_result: ValidateVerificationResult) -> Self {
        ValidateEmailVerificationResponseData {}
    }
}
