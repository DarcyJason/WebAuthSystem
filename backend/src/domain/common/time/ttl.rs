use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTL(Duration);

impl TTL {
    pub fn from_seonds(seconds: u64) -> Self {
        TTL(Duration::from_secs(seconds))
    }
    pub fn value(&self) -> &Duration {
        &self.0
    }
}
