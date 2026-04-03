use crate::domain::error::DomainResult;
use crate::infrastructure::layered::cache_layer::CacheLayer;
use crate::infrastructure::layered::cache_store::CacheStore;
use async_trait::async_trait;
use moka::future::Cache;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct MokaClient {
    pub connection: Cache<String, String>,
}

impl MokaClient {
    pub fn new() -> Self {
        let connection: Cache<String, String> = Cache::builder()
            .max_capacity(10_000)
            .time_to_live(Duration::from_secs(300))
            .build();
        MokaClient { connection }
    }
}

#[async_trait]
impl CacheStore for MokaClient {
    fn layer(&self) -> CacheLayer {
        CacheLayer::L1Moka
    }
    async fn get(&self, key: &str) -> DomainResult<Option<String>> {
        Ok(self.connection.get(key).await)
    }
    async fn batch_set(&self, entries: Vec<(String, String)>) -> DomainResult<()> {
        let tasks: Vec<_> = entries
            .into_iter()
            .map(|(k, v)| {
                let conn = self.connection.clone();
                tokio::spawn(async move { conn.insert(k, v).await })
            })
            .collect();
        futures_util::future::join_all(tasks).await;
        Ok(())
    }
}
