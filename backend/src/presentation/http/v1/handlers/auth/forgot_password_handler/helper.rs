use crate::application::commands::auth::forgot_password_command::ForgotPasswordCommand;
use crate::application::results::commands_results::auth::forgot_password_result::ForgotPasswordResult;
use crate::domain::user::value_objects::user_email::{UserEmail, UserEmailError};
use crate::presentation::http::v1::errors::ApiError;
use crate::presentation::http::v1::handlers::auth::forgot_password_handler::request::ForgotPasswordRequestPayload;
use crate::presentation::http::v1::handlers::auth::forgot_password_handler::response::ForgotPasswordResponseData;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ForgotPasswordRequestPayloadError {
    #[error("email is required")]
    UserEmailRequired,
    #[error("email is invalid")]
    UserEmailInvalid,
}

impl TryFrom<ForgotPasswordRequestPayload> for ForgotPasswordCommand {
    type Error = ForgotPasswordRequestPayloadError;
    fn try_from(payload: ForgotPasswordRequestPayload) -> Result<Self, Self::Error> {
        let email = UserEmail::new(payload.email).map_err(|e| match e {
            UserEmailError::UserEmailRequired => {
                ForgotPasswordRequestPayloadError::UserEmailRequired
            }
            UserEmailError::UserEmailInvalid => ForgotPasswordRequestPayloadError::UserEmailInvalid,
        })?;
        Ok(ForgotPasswordCommand { email })
    }
}

impl From<ForgotPasswordRequestPayloadError> for ApiError {
    fn from(e: ForgotPasswordRequestPayloadError) -> Self {
        ApiError::BadRequest {
            message: e.to_string(),
        }
    }
}

impl From<ForgotPasswordResult> for ForgotPasswordResponseData {
    fn from(_result: ForgotPasswordResult) -> Self {
        ForgotPasswordResponseData {}
    }
}
