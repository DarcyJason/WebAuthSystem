use std::sync::Arc;

use crate::application::{
    commands::auth::validate_verification_command::ValidateVerificationCommand,
    errors::{AppError, AppResult},
    results::commands_results::auth::validate_verification_result::ValidateVerificationResult,
};
use crate::domain::auth::repositories::email_verification_token_repository::EmailVerificationTokenRepository;
use crate::domain::user::repositories::user_repository::UserRepository;

pub struct ValidateVerificationCase {
    auth_email_verification_cache: Arc<dyn EmailVerificationTokenRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl ValidateVerificationCase {
    pub fn new(
        auth_email_verification_cache: Arc<dyn EmailVerificationTokenRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        ValidateVerificationCase {
            auth_email_verification_cache,
            user_repo,
        }
    }
    pub async fn execute(
        &self,
        validate_email_verification_command: ValidateVerificationCommand,
    ) -> AppResult<ValidateVerificationResult> {
        let user_email = validate_email_verification_command.email;
        let verification_token = self
            .auth_email_verification_cache
            .get_email_verification_token(&user_email)
            .await?;
        let verification_token = match verification_token {
            Some(verification_token) => verification_token,
            None => return Err(AppError::EmailVerificationTokenNotFound),
        };
        if validate_email_verification_command.verification_token != verification_token {
            return Err(AppError::EmailVerificationTokenInvalid);
        }
        let updated_user = self.user_repo.update_status_as_true(&user_email).await?;
        if updated_user.is_none() {
            return Err(AppError::UserNotFound);
        }
        self.auth_email_verification_cache
            .delete_email_verification_token(&user_email)
            .await?;
        Ok(ValidateVerificationResult {})
    }
}
