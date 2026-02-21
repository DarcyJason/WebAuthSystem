use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    application::{
        commands::auth::validate_email_verification_command::ValidateEmailVerificationCommand,
        errors::AppResult,
        results::commands_results::auth::validate_email_verification_result::ValidateEmailVerificationResult,
    },
    domain::auth::repositories::cache::email_verification_cache::EmailVerificationCache,
};

pub struct ValidateEmailVerificationCase {
    auth_email_verification_cache: Arc<Mutex<dyn EmailVerificationCache>>,
}

impl ValidateEmailVerificationCase {
    pub fn new(auth_email_verification_cache: Arc<Mutex<dyn EmailVerificationCache>>) -> Self {
        ValidateEmailVerificationCase {
            auth_email_verification_cache,
        }
    }
    pub async fn execute(
        &self,
        validate_email_verification_command: ValidateEmailVerificationCommand,
    ) -> AppResult<ValidateEmailVerificationResult> {
        Ok(ValidateEmailVerificationResult {})
    }
}
