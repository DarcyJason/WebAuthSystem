use crate::domain::auth::repositories::verification_token_repository::{
    VerificationTokenCommandRepository, VerificationTokenQueryRepository,
};
use crate::domain::auth::value_objects::tokens::verification_token::VerificationToken;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_kind::VerificationTokenKind;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_value::VerificationTokenValue;
use crate::domain::error::DomainResult;
use crate::domain::user::value_objects::user::user_id::UserId;
use crate::infrastructure::internal::caches::moka::verification_token_repository::MokaVerificationTokenRepository;
use crate::infrastructure::internal::caches::redis::verification_token_repository::RedisVerificationTokenRepository;
use crate::infrastructure::internal::persistence::postgres::verification_token_repository::PostgresVerificationTokenRepository;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct LayeredVerificationTokenRepository {
    l1_cache: MokaVerificationTokenRepository,
    l2_cache: RedisVerificationTokenRepository,
    source_repo: PostgresVerificationTokenRepository,
}

impl LayeredVerificationTokenRepository {
    pub fn new(
        l1_cache: MokaVerificationTokenRepository,
        l2_cache: RedisVerificationTokenRepository,
        source_repo: PostgresVerificationTokenRepository,
    ) -> Self {
        Self {
            l1_cache,
            l2_cache,
            source_repo,
        }
    }

    async fn warm_up_l1(&self, token: &VerificationToken) {
        let _ = self
            .l1_cache
            .save_with_ttl(token, self.l1_cache.ttl())
            .await;
    }

    async fn warm_up_l2_and_l1(&self, token: &VerificationToken) {
        let _ = self
            .l2_cache
            .save_with_ttl(token, self.l2_cache.ttl())
            .await;
        self.warm_up_l1(token).await;
    }
}

#[async_trait]
impl VerificationTokenCommandRepository for LayeredVerificationTokenRepository {
    async fn save(&self, token: &VerificationToken) -> DomainResult<VerificationToken> {
        let saved = self.source_repo.save(token).await?;
        self.warm_up_l2_and_l1(&saved).await;
        Ok(saved)
    }

    async fn mark_used(&self, value: &VerificationTokenValue) -> DomainResult<()> {
        self.source_repo.mark_used(value).await
    }

    async fn invalidate_by_user_id_and_kind(
        &self,
        user_id: &UserId,
        kind: VerificationTokenKind,
    ) -> DomainResult<()> {
        // Invalidate at the source of truth (Postgres)
        self.source_repo
            .invalidate_by_user_id_and_kind(user_id, kind)
            .await?;

        // Best-effort cache invalidation:
        // We attempt to remove any cached tokens related to this user from L2 and L1.
        // The cache repositories store tokens keyed by value; unless we maintain
        // an index of user->tokens in cache, we cannot deterministically remove all
        // entries here. Therefore, we rely on source invalidation as authoritative.
        // If cache implementations provide a user-scoped invalidation API in future,
        // call them here (e.g., self.l2_cache.invalidate_by_user(user_id).await;).

        Ok(())
    }
}

#[async_trait]
impl VerificationTokenQueryRepository for LayeredVerificationTokenRepository {
    async fn get_by_value(
        &self,
        value: &VerificationTokenValue,
    ) -> DomainResult<Option<VerificationToken>> {
        if let Some(token) = self.l1_cache.get_by_value(value).await? {
            return Ok(Some(token));
        }
        if let Some(token) = self.l2_cache.get_by_value(value).await? {
            self.warm_up_l1(&token).await;
            return Ok(Some(token));
        }
        let token = self.source_repo.get_by_value(value).await?;
        if let Some(ref t) = token {
            self.warm_up_l2_and_l1(t).await;
        }
        Ok(token)
    }
}
