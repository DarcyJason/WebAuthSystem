use std::sync::Arc;
use tokio::sync::Mutex;

use crate::application::{
    commands::auth::validate_verification_command::ValidateVerificationCommand,
    errors::{AppError, AppResult},
    results::commands_results::auth::validate_verification_result::ValidateVerificationResult,
};
use crate::domain::auth::repositories::email_verification_token_repository::EmailVerificationTokenRepository;

pub struct ValidateVerificationCase {
    auth_email_verification_cache: Arc<Mutex<dyn EmailVerificationTokenRepository>>,
}

impl ValidateVerificationCase {
    pub fn new(
        auth_email_verification_cache: Arc<Mutex<dyn EmailVerificationTokenRepository>>,
    ) -> Self {
        ValidateVerificationCase {
            auth_email_verification_cache,
        }
    }
    pub async fn execute(
        &self,
        validate_email_verification_command: ValidateVerificationCommand,
    ) -> AppResult<ValidateVerificationResult> {
        let verification_token = self
            .auth_email_verification_cache
            .lock()
            .await
            .get_email_verification_token(validate_email_verification_command.email)
            .await
            .map_err(|_| AppError::GetEmailVerificationTokenFailed)?;
        let verification_token = match verification_token {
            Some(verification_token) => verification_token,
            None => return Err(AppError::EmailVerificationTokenNotFound),
        };
        if validate_email_verification_command.verification_token != verification_token {
            return Err(AppError::EmailVerificationTokenInvalid);
        }
        Ok(ValidateVerificationResult {})
    }
}
