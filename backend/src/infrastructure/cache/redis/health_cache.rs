use async_trait::async_trait;

use crate::{
    domain::{
        error::{DomainError, RepoResult},
        health::repositories::HealthCache,
    },
    infrastructure::cache::redis::client::RedisClient,
};

#[derive(Debug, Clone)]
pub struct RedisHealthCache {
    redis: RedisClient,
}

impl RedisHealthCache {
    pub fn new(redis: RedisClient) -> Self {
        RedisHealthCache { redis }
    }
}

#[async_trait]
impl HealthCache for RedisHealthCache {
    async fn check(&self) -> RepoResult<()> {
        let result: String = redis::cmd("PING")
            .query_async(&mut self.redis.client.clone())
            .await
            .map_err(|_| DomainError::RepositoryError)?;
        if result == "PONG" {
            Ok(())
        } else {
            Err(DomainError::RepositoryError)
        }
    }
}
