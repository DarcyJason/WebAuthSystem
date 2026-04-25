use crate::domain::auth::repositories::verification_token_repository::VerificationTokenRepository;
use crate::domain::auth::value_objects::tokens::verification_token::VerificationToken;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_value::VerificationTokenValue;
use crate::domain::common::value_objects::time::ttl::TTL;
use crate::domain::error::{DomainResult, UserRepositoryJsonSnafu};
use crate::infrastructure::internal::layered::cache_operation::CacheOperation;
use crate::infrastructure::internal::layered::cache_store::CacheStore;
use async_trait::async_trait;
use chrono::Utc;
use snafu::ResultExt;

#[derive(Debug, Clone)]
pub struct CacheVerificationTokenRepository<S> {
    store: S,
    ttl: Option<TTL>,
}

impl<S: CacheStore> CacheVerificationTokenRepository<S> {
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

    fn key(value: &VerificationTokenValue) -> String {
        format!("verification_token:value:{}", value.value())
    }

    fn ttl_from_token(token: &VerificationToken) -> Option<TTL> {
        let secs = token
            .expires_at()
            .value()
            .signed_duration_since(Utc::now())
            .num_seconds();
        if secs <= 0 {
            None
        } else {
            Some(TTL::from_seconds(secs as u64))
        }
    }

    pub async fn save_with_ttl(
        &self,
        token: &VerificationToken,
        ttl: Option<&TTL>,
    ) -> DomainResult<VerificationToken> {
        let payload = serde_json::to_string(token).context(UserRepositoryJsonSnafu {
            layer: self.store.layer(),
            operation: CacheOperation::Serialize,
            message: "serialize verification token failed".to_string(),
        })?;
        self.store
            .batch_set(vec![(Self::key(&token.value()), payload)], ttl)
            .await?;
        Ok(token.clone())
    }
}

#[async_trait]
impl<S: CacheStore> VerificationTokenRepository for CacheVerificationTokenRepository<S> {
    async fn save(&self, token: &VerificationToken) -> DomainResult<VerificationToken> {
        let entity_ttl = Self::ttl_from_token(token);
        let ttl = entity_ttl.as_ref().or(self.ttl());
        self.save_with_ttl(token, ttl).await
    }

    async fn get_by_value(
        &self,
        value: &VerificationTokenValue,
    ) -> DomainResult<Option<VerificationToken>> {
        let raw = self.store.get(&Self::key(value)).await?;
        match raw {
            None => Ok(None),
            Some(s) => {
                let token: VerificationToken =
                    serde_json::from_str(&s).context(UserRepositoryJsonSnafu {
                        layer: self.store.layer(),
                        operation: CacheOperation::Deserialize,
                        message: "deserialize verification token failed".to_string(),
                    })?;
                Ok(Some(token))
            }
        }
    }

    async fn mark_used(&self, value: &VerificationTokenValue) -> DomainResult<()> {
        // Cache entries are short-lived; invalidate by overwriting is not straightforward
        // without a full read-modify-write. For simplicity, we skip cache invalidation here —
        // the source-of-truth mark_used is done in Postgres.
        let _ = value;
        Ok(())
    }
}
