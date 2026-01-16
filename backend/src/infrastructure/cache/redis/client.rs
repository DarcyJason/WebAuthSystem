use crate::infrastructure::config::redis::RedisConfig;
use redis::RedisError;

#[derive(Debug, Clone)]
pub struct RedisClient {
    pub client: redis::aio::MultiplexedConnection,
}

impl RedisClient {
    pub async fn new(config: &RedisConfig) -> Result<Self, RedisError> {
        let client = redis::Client::open(config.address.clone())?;
        let con = client.get_multiplexed_async_connection().await?;
        Ok(RedisClient { client: con })
    }
}
