use crate::domain::auth::entities::refresh_token::RefreshTokenEntity;
use crate::domain::auth::value_objects::tokens::refresh_token_hash::RefreshTokenHash;
use crate::domain::error::DomainResult;
use async_trait::async_trait;

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn save(&self, refresh_token: &RefreshTokenEntity) -> DomainResult<RefreshTokenEntity>;
    async fn get_by_hash(
        &self,
        hash: &RefreshTokenHash,
    ) -> DomainResult<Option<RefreshTokenEntity>>;
}
