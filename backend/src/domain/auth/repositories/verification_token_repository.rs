use crate::domain::auth::value_objects::tokens::verification_token::VerificationToken;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_value::VerificationTokenValue;
use crate::domain::error::DomainResult;
use async_trait::async_trait;

#[async_trait]
pub trait VerificationTokenRepository: Send + Sync {
    async fn save(&self, token: &VerificationToken) -> DomainResult<VerificationToken>;
    async fn get_by_value(
        &self,
        value: &VerificationTokenValue,
    ) -> DomainResult<Option<VerificationToken>>;
    async fn mark_used(&self, value: &VerificationTokenValue) -> DomainResult<()>;
}
