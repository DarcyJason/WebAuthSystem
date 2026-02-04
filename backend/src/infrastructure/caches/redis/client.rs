use crate::infrastructure::config::redis_config::RedisConfig;
use anyhow::Context;

#[derive(Debug, Clone)]
pub struct RedisClient {
    pub client: redis::aio::MultiplexedConnection,
}

impl RedisClient {
    pub async fn new(config: &RedisConfig) -> anyhow::Result<Self> {
        let client = redis::Client::open(config.address.clone())
            .context("connect to redis server failed")?;
        let con = client
            .get_multiplexed_async_connection()
            .await
            .context("get async redis client failed")?;
        Ok(RedisClient { client: con })
    }
}
