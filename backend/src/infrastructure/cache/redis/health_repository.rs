use async_trait::async_trait;

use crate::{
    domain::{
        error::{DomainError, RepoResult},
        health::repositories::HealthRepository,
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
            .map_err(|_| DomainError::RepositoryError)?;
        if result == "PONG" {
            Ok(())
        } else {
            Err(DomainError::RepositoryError)
        }
    }
}
