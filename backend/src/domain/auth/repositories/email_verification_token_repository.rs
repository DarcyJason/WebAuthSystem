use crate::domain::auth::value_objects::verification_token::VerificationToken;
use crate::domain::common::time::ttl::TTL;
use crate::domain::user::value_objects::user_email::UserEmail;
use async_trait::async_trait;

pub enum EmailVerificationTokenRepositoryError {
    TokenStoreUnavailable,
    TokenNotFound,
    TokenRemoveFailed,
}

#[async_trait]
pub trait EmailVerificationTokenRepository: Send + Sync {
    async fn save_email_verification_token(
        &mut self,
        user_email: UserEmail,
        mail_token: VerificationToken,
        ttl: TTL,
    ) -> Result<(), EmailVerificationTokenRepositoryError>;
    async fn get_email_verification_token(
        &mut self,
        user_email: UserEmail,
    ) -> Result<Option<VerificationToken>, EmailVerificationTokenRepositoryError>;
    async fn delete_email_verification_token(
        &mut self,
        user_email: UserEmail,
    ) -> Result<(), EmailVerificationTokenRepositoryError>;
}
