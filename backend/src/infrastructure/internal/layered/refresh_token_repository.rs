use crate::domain::auth::entities::refresh_token::RefreshTokenEntity;
use crate::domain::auth::repositories::refresh_token_repository::RefreshTokenRepository;
use crate::domain::auth::value_objects::tokens::refresh_token_hash::RefreshTokenHash;
use crate::domain::error::DomainResult;
use crate::infrastructure::internal::caches::moka::refresh_token_repository::MokaRefreshTokenRepository;
use crate::infrastructure::internal::caches::redis::refresh_token_repository::RedisRefreshTokenRepository;
use crate::infrastructure::internal::persistence::postgres::refresh_token_repository::PostgresRefreshTokenRepository;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct LayeredRefreshTokenRepository {
    l1_cache: MokaRefreshTokenRepository,
    l2_cache: RedisRefreshTokenRepository,
    source_repo: PostgresRefreshTokenRepository,
}

impl LayeredRefreshTokenRepository {
    pub fn new(
        l1_cache: MokaRefreshTokenRepository,
        l2_cache: RedisRefreshTokenRepository,
        source_repo: PostgresRefreshTokenRepository,
    ) -> Self {
        Self {
            l1_cache,
            l2_cache,
            source_repo,
        }
    }

    async fn warm_up_l1(&self, refresh_token: &RefreshTokenEntity) {
        let _ = self
            .l1_cache
            .save_with_ttl(refresh_token, self.l1_cache.ttl())
            .await;
    }

    async fn warm_up_l2_and_l1(&self, refresh_token: &RefreshTokenEntity) {
        let _ = self
            .l2_cache
            .save_with_ttl(refresh_token, self.l2_cache.ttl())
            .await;
        self.warm_up_l1(refresh_token).await;
    }
}

#[async_trait]
impl RefreshTokenRepository for LayeredRefreshTokenRepository {
    async fn save(&self, refresh_token: &RefreshTokenEntity) -> DomainResult<RefreshTokenEntity> {
        let saved = self.source_repo.save(refresh_token).await?;
        self.warm_up_l2_and_l1(&saved).await;
        Ok(saved)
    }

    async fn get_by_hash(
        &self,
        hash: &RefreshTokenHash,
    ) -> DomainResult<Option<RefreshTokenEntity>> {
        if let Some(entity) = self.l1_cache.get_by_hash(hash).await? {
            return Ok(Some(entity));
        }
        if let Some(entity) = self.l2_cache.get_by_hash(hash).await? {
            self.warm_up_l1(&entity).await;
            return Ok(Some(entity));
        }
        let entity = self.source_repo.get_by_hash(hash).await?;
        if let Some(ref e) = entity {
            self.warm_up_l2_and_l1(e).await;
        }
        Ok(entity)
    }
}
