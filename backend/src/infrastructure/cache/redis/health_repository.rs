use async_trait::async_trait;

use crate::{
    domain::health::{errors::HealthError, repositories::HealthRepository},
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
    async fn check(&self) -> Result<(), HealthError> {
        let result: String = redis::cmd("PING")
            .query_async(&mut self.redis.client.clone())
            .await?;
        if result == "PONG" {
            Ok(())
        } else {
            Err(HealthError::RedisIsUnhealthy)
        }
    }
}
