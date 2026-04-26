use crate::domain::auth::value_objects::tokens::verification_token::VerificationToken;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_kind::VerificationTokenKind;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_value::VerificationTokenValue;
use crate::domain::error::DomainResult;
use crate::domain::user::value_objects::user::user_id::UserId;
use async_trait::async_trait;

#[async_trait]
pub trait VerificationTokenRepository: Send + Sync {
    async fn save(&self, token: &VerificationToken) -> DomainResult<VerificationToken>;
    async fn get_by_value(
        &self,
        value: &VerificationTokenValue,
    ) -> DomainResult<Option<VerificationToken>>;
    async fn mark_used(&self, value: &VerificationTokenValue) -> DomainResult<()>;

    /// Invalidate all verification tokens for a given user and token kind.
    /// This will typically mark existing tokens as `Invalid` (database-side)
    /// so that subsequent verification attempts using older tokens are rejected.
    async fn invalidate_by_user_id_and_kind(
        &self,
        user_id: &UserId,
        kind: VerificationTokenKind,
    ) -> DomainResult<()>;
}
