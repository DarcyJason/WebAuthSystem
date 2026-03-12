use crate::domain::auth::value_objects::verification_token::VerificationToken;
use crate::domain::common::time::ttl::TTL;
use crate::domain::user::value_objects::user_email::UserEmail;
use crate::infrastructure::errors::email_verification_token_repository_error::EmailVerificationTokenRepositoryError;
use async_trait::async_trait;

#[async_trait]
pub trait EmailVerificationTokenRepository: Send + Sync {
    async fn save(
        &self,
        user_email: &UserEmail,
        mail_token: VerificationToken,
        ttl: TTL,
    ) -> Result<(), EmailVerificationTokenRepositoryError>;
    async fn get_by_user_email(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<VerificationToken>, EmailVerificationTokenRepositoryError>;
    async fn delete(
        &self,
        user_email: &UserEmail,
    ) -> Result<(), EmailVerificationTokenRepositoryError>;
}
