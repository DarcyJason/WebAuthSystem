use crate::infrastructure::cache::redis::{client::RedisClient, errors::RedisError};

#[derive(Debug, Clone)]
pub struct RedisHealthCache {
    redis: RedisClient,
}

impl RedisHealthCache {
    pub fn new(redis: RedisClient) -> Self {
        RedisHealthCache { redis }
    }
    pub async fn check(&self) -> Result<(), RedisError> {
        let result: String = redis::cmd("PING")
            .query_async(&mut self.redis.client.clone())
            .await
            .map_err(|_| RedisError::ExecuteCommandError)?;
        if result == "PONG" {
            Ok(())
        } else {
            Err(RedisError::ConnectionError)
        }
    }
}
