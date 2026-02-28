use crate::domain::auth::value_objects::verification_token::VerificationToken;
use crate::domain::common::time::ttl::TTL;
use crate::domain::user::value_objects::user_email::UserEmail;
use async_trait::async_trait;

pub enum EmailVerificationCacheError {
    TokenStoreUnavailable,
    TokenNotFound,
    TokenRemoveFailed,
}

#[async_trait]
pub trait EmailVerificationCache: Send + Sync {
    async fn save_email_verification_token(
        &mut self,
        user_email: UserEmail,
        mail_token: VerificationToken,
        ttl: TTL,
    ) -> Result<(), EmailVerificationCacheError>;
    async fn get_email_verification_token(
        &mut self,
        user_email: UserEmail,
    ) -> Result<Option<VerificationToken>, EmailVerificationCacheError>;
    async fn delete_email_verification_token(
        &mut self,
        user_email: UserEmail,
    ) -> Result<(), EmailVerificationCacheError>;
}
