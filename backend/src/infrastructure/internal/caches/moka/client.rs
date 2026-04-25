use crate::domain::common::value_objects::time::ttl::TTL;
use crate::domain::error::DomainResult;
use crate::infrastructure::internal::layered::cache_layer::CacheLayer;
use crate::infrastructure::internal::layered::cache_store::CacheStore;
use async_trait::async_trait;
use moka::Expiry;
use moka::future::Cache;
use std::time::{Duration, Instant};

type MokaValue = (Option<Duration>, String);

struct MokaExpiry;

impl Expiry<String, MokaValue> for MokaExpiry {
    fn expire_after_create(
        &self,
        _key: &String,
        value: &MokaValue,
        _created_at: Instant,
    ) -> Option<Duration> {
        value.0
    }
}

#[derive(Debug, Clone)]
pub struct MokaClient {
    pub connection: Cache<String, MokaValue>,
}

impl MokaClient {
    pub fn new() -> Self {
        let connection: Cache<String, MokaValue> = Cache::builder()
            .max_capacity(10_000)
            .time_to_live(Duration::from_secs(300))
            .expire_after(MokaExpiry)
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
        Ok(self.connection.get(key).await.map(|(_, v)| v))
    }
    async fn batch_set(
        &self,
        entries: Vec<(String, String)>,
        ttl: Option<&TTL>,
    ) -> DomainResult<()> {
        let expiry = ttl.map(|t| *t.value());
        let tasks: Vec<_> = entries
            .into_iter()
            .map(|(k, v)| {
                let conn = self.connection.clone();
                tokio::spawn(async move { conn.insert(k, (expiry, v)).await })
            })
            .collect();
        futures_util::future::join_all(tasks).await;
        Ok(())
    }
}
