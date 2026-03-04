use std::hash::Hash;
use std::time::Duration;

use moka::future::Cache;

#[derive(Clone)]
pub struct MokaClient<K, V> {
    pub client: Cache<K, V>,
}

impl<K, V> MokaClient<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        MokaClient {
            client: Cache::builder()
                .max_capacity(10_000)
                .time_to_live(Duration::from_secs(300))
                .build(),
        }
    }
}

impl<K, V> Default for MokaClient<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
