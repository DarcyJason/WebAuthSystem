use std::sync::Arc;

use tokio::sync::Mutex;

use crate::domain::auth::repositories::cache::email_verification_cache::EmailVerificationCache;

pub struct ValidateEmailVerificationCase {
    auth_email_verification_cache: Arc<Mutex<dyn EmailVerificationCache>>,
}

impl ValidateEmailVerificationCase {
    pub fn new(auth_email_verification_cache: Arc<Mutex<dyn EmailVerificationCache>>) -> Self {
        ValidateEmailVerificationCase {
            auth_email_verification_cache,
        }
    }
}
