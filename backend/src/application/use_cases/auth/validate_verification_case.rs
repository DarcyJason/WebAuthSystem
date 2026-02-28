use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    application::{
        commands::auth::validate_verification_command::ValidateVerificationCommand,
        errors::AppResult,
        results::commands_results::auth::validate_verification_result::ValidateVerificationResult,
    },
    domain::auth::repositories::cache::email_verification_cache::EmailVerificationCache,
};

pub struct ValidateVerificationCase {
    auth_email_verification_cache: Arc<Mutex<dyn EmailVerificationCache>>,
}

impl ValidateVerificationCase {
    pub fn new(auth_email_verification_cache: Arc<Mutex<dyn EmailVerificationCache>>) -> Self {
        ValidateVerificationCase {
            auth_email_verification_cache,
        }
    }
    pub async fn execute(
        &self,
        _validate_email_verification_command: ValidateVerificationCommand,
    ) -> AppResult<ValidateVerificationResult> {
        Ok(ValidateVerificationResult {})
    }
}
