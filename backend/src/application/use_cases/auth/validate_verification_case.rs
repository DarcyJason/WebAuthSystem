use std::sync::Arc;

use crate::application::{
    commands::auth::validate_verification_command::ValidateVerificationCommand,
    errors::{CaseError, CaseResult},
    results::commands_results::auth::validate_verification_result::ValidateVerificationResult,
};
use crate::domain::auth::repositories::email_verification_token_repository::EmailVerificationTokenRepository;
use crate::domain::user::repositories::user_repository::UserRepository;
use crate::infrastructure::errors::InfraError;

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
    ) -> CaseResult<ValidateVerificationResult> {
        let user_email = validate_email_verification_command.email;
        let verification_token = self
            .auth_email_verification_cache
            .get_by_user_email(&user_email)
            .await
            .map_err(InfraError::from)?;
        let verification_token = match verification_token {
            Some(verification_token) => verification_token,
            None => return Err(CaseError::EmailVerificationTokenNotFound),
        };
        if validate_email_verification_command.verification_token != verification_token {
            return Err(CaseError::EmailVerificationTokenInvalid);
        }
        let updated_user = self
            .user_repo
            .update_status_as_true(&user_email)
            .await
            .map_err(InfraError::from)?;
        if updated_user.is_none() {
            return Err(CaseError::UserNotFound);
        }
        self.auth_email_verification_cache
            .delete(&user_email)
            .await
            .map_err(InfraError::from)?;
        Ok(ValidateVerificationResult {})
    }
}
