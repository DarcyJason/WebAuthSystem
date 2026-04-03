use crate::domain::error::{DomainResult, UserRepositoryRedisSnafu};
use crate::infrastructure::layered::cache_layer::CacheLayer;
use crate::infrastructure::layered::cache_operation::CacheOperation;
use crate::infrastructure::layered::cache_store::CacheStore;
use crate::infrastructure::{
    config::redis_config::RedisConfig,
    error::{InfrastructureResult, RedisSnafu},
};
use async_trait::async_trait;
use snafu::ResultExt;

#[derive(Debug, Clone)]
pub struct RedisClient {
    pub connection: redis::aio::MultiplexedConnection,
}

impl RedisClient {
    pub async fn new(config: &RedisConfig) -> InfrastructureResult<Self> {
        let client = redis::Client::open(config.address.clone()).context(RedisSnafu)?;
        let connection = client
            .get_multiplexed_async_connection()
            .await
            .context(RedisSnafu)?;
        Ok(RedisClient { connection })
    }
}

#[async_trait]
impl CacheStore for RedisClient {
    fn layer(&self) -> CacheLayer {
        CacheLayer::L2Redis
    }
    async fn get(&self, key: &str) -> DomainResult<Option<String>> {
        let mut conn = self.connection.clone();
        redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await
            .context(UserRepositoryRedisSnafu {
                layer: CacheLayer::L2Redis,
                operation: CacheOperation::Get,
                message: "cache get failed".to_string(),
            })
    }
    async fn batch_set(&self, entries: Vec<(String, String)>) -> DomainResult<()> {
        let mut conn = self.connection.clone();
        let mut pipe = redis::pipe();
        for (k, v) in &entries {
            pipe.cmd("SET").arg(k).arg(v);
        }
        pipe.query_async::<()>(&mut conn)
            .await
            .context(UserRepositoryRedisSnafu {
                layer: CacheLayer::L2Redis,
                operation: CacheOperation::BatchSet,
                message: "cache batch_set failed".to_string(),
            })
    }
}
