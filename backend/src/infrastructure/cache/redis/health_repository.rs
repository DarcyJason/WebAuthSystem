use async_trait::async_trait;

use crate::{
    domain::{
        error::{DomainError, RepoResult},
        health::{errors::HealthError, repositories::HealthRepository},
    },
    infrastructure::cache::redis::client::RedisClient,
};

#[derive(Debug, Clone)]
pub struct RedisHealthRepository {
    redis: RedisClient,
}

impl RedisHealthRepository {
    pub fn new(redis: RedisClient) -> Self {
        RedisHealthRepository { redis }
    }
}

#[async_trait]
impl HealthRepository for RedisHealthRepository {
    async fn check(&self) -> RepoResult<()> {
        let result: String = redis::cmd("PING")
            .query_async(&mut self.redis.client.clone())
            .await
            .map_err(|e| DomainError::Validation(e.to_string()))?;
        if result == "PONG" {
            Ok(())
        } else {
            Err(DomainError::Validation(
                HealthError::RedisIsUnhealthy.to_string(),
            ))
        }
    }
}
