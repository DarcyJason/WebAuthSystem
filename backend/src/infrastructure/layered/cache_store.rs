use crate::domain::error::DomainResult;
use crate::infrastructure::layered::cache_layer::CacheLayer;
use async_trait::async_trait;

#[async_trait]
pub trait CacheStore: Send + Sync {
    fn layer(&self) -> CacheLayer;
    async fn get(&self, key: &str) -> DomainResult<Option<String>>;
    async fn batch_set(&self, entries: Vec<(String, String)>) -> DomainResult<()>;
}
