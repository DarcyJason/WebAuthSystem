use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::health::errors::HealthError;
use crate::infrastructure::cache::redis::health_cache::RedisHealthCache;
use async_trait::async_trait;

#[async_trait]
pub trait HealthCache: Send + Sync + 'static {
    async fn check(&self) -> DomainResult<()>;
}

pub struct RedisHealthCacheAdapter {
    inner: RedisHealthCache,
}

impl RedisHealthCacheAdapter {
    pub fn new(inner: RedisHealthCache) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl HealthCache for RedisHealthCacheAdapter {
    async fn check(&self) -> DomainResult<()> {
        self.inner
            .check()
            .await
            .map_err(|_| DomainError::HealthError(HealthError::RedisConnectionError))
    }
}
