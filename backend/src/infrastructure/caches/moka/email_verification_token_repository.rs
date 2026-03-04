use async_trait::async_trait;

use crate::domain::auth::repositories::email_verification_token_repository::{
    EmailVerificationTokenRepository, EmailVerificationTokenRepositoryError,
};
use crate::domain::auth::value_objects::verification_token::VerificationToken;
use crate::domain::common::time::ttl::TTL;
use crate::domain::user::value_objects::user_email::UserEmail;

pub struct MokaEmailVerificationTokenRepository;

impl MokaEmailVerificationTokenRepository {
    pub fn new() -> Self {
        MokaEmailVerificationTokenRepository
    }
}

impl Default for MokaEmailVerificationTokenRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmailVerificationTokenRepository for MokaEmailVerificationTokenRepository {
    async fn save_email_verification_token(
        &mut self,
        _user_email: UserEmail,
        _mail_token: VerificationToken,
        _ttl: TTL,
    ) -> Result<(), EmailVerificationTokenRepositoryError> {
        Err(EmailVerificationTokenRepositoryError::TokenStoreUnavailable)
    }

    async fn get_email_verification_token(
        &mut self,
        _user_email: UserEmail,
    ) -> Result<Option<VerificationToken>, EmailVerificationTokenRepositoryError> {
        Err(EmailVerificationTokenRepositoryError::TokenNotFound)
    }

    async fn delete_email_verification_token(
        &mut self,
        _user_email: UserEmail,
    ) -> Result<(), EmailVerificationTokenRepositoryError> {
        Err(EmailVerificationTokenRepositoryError::TokenRemoveFailed)
    }
}
