use async_trait::async_trait;

use crate::{
    domain::health::repositories::HealthCache,
    infrastructure::{
        cache::redis::{client::RedisClient, errors::RedisError},
        errors::InfraResult,
    },
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
    async fn check(&self) -> InfraResult<()> {
        let result: String = redis::cmd("PING")
            .query_async(&mut self.redis.client.clone())
            .await
            .map_err(|_| RedisError::ExecuteCommandError)?;
        if result == "PONG" {
            Ok(())
        } else {
            Err(RedisError::ConnectionError.into())
        }
    }
}
