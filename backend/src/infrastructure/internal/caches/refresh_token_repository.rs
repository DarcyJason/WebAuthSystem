use crate::domain::auth::entities::refresh_token::RefreshTokenEntity;
use crate::domain::auth::repositories::refresh_token_repository::RefreshTokenRepository;
use crate::domain::auth::value_objects::tokens::refresh_token_hash::RefreshTokenHash;
use crate::domain::common::value_objects::time::ttl::TTL;
use crate::domain::error::{DomainResult, UserRepositoryJsonSnafu};
use crate::infrastructure::internal::layered::cache_operation::CacheOperation;
use crate::infrastructure::internal::layered::cache_store::CacheStore;
use async_trait::async_trait;
use chrono::Utc;
use snafu::ResultExt;

#[derive(Debug, Clone)]
pub struct CacheRefreshTokenRepository<S> {
    store: S,
    ttl: Option<TTL>,
}

impl<S: CacheStore> CacheRefreshTokenRepository<S> {
    pub fn new(store: S) -> Self {
        Self { store, ttl: None }
    }

    pub fn with_ttl(store: S, ttl: TTL) -> Self {
        Self {
            store,
            ttl: Some(ttl),
        }
    }

    pub fn ttl(&self) -> Option<&TTL> {
        self.ttl.as_ref()
    }

    fn key_by_hash(refresh_token: &RefreshTokenEntity) -> String {
        format!("refresh_token:hash:{}", refresh_token.token_hash().value())
    }

    fn ttl_from_entity(refresh_token: &RefreshTokenEntity) -> Option<TTL> {
        let seconds = refresh_token
            .expires_at()
            .value()
            .signed_duration_since(Utc::now())
            .num_seconds();
        if seconds <= 0 {
            return None;
        }
        Some(TTL::from_seconds(seconds as u64))
    }

    async fn serialize(
        &self,
        refresh_token: &RefreshTokenEntity,
        operation: CacheOperation,
    ) -> DomainResult<String> {
        serde_json::to_string(refresh_token).context(UserRepositoryJsonSnafu {
            layer: self.store.layer(),
            operation,
            message: "serialize refresh token failed".to_string(),
        })
    }

    pub async fn save_with_ttl(
        &self,
        refresh_token: &RefreshTokenEntity,
        ttl: Option<&TTL>,
    ) -> DomainResult<RefreshTokenEntity> {
        let payload = self
            .serialize(refresh_token, CacheOperation::Serialize)
            .await?;
        self.store
            .batch_set(vec![(Self::key_by_hash(refresh_token), payload)], ttl)
            .await?;
        Ok(refresh_token.clone())
    }
}

#[async_trait]
impl<S: CacheStore> RefreshTokenRepository for CacheRefreshTokenRepository<S> {
    async fn save(&self, refresh_token: &RefreshTokenEntity) -> DomainResult<RefreshTokenEntity> {
        let entity_ttl = Self::ttl_from_entity(refresh_token);
        let ttl = entity_ttl.as_ref().or(self.ttl());
        self.save_with_ttl(refresh_token, ttl).await
    }

    async fn get_by_hash(
        &self,
        hash: &RefreshTokenHash,
    ) -> DomainResult<Option<RefreshTokenEntity>> {
        let key = format!("refresh_token:hash:{}", hash.value());
        match self.store.get(&key).await? {
            None => Ok(None),
            Some(s) => {
                let entity: RefreshTokenEntity =
                    serde_json::from_str(&s).context(UserRepositoryJsonSnafu {
                        layer: self.store.layer(),
                        operation: CacheOperation::Deserialize,
                        message: "deserialize refresh token failed".to_string(),
                    })?;
                Ok(Some(entity))
            }
        }
    }
}
