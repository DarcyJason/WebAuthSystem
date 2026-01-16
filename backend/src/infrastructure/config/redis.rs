use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub address: String,
}

impl Default for RedisConfig {
    fn default() -> Self {
        RedisConfig {
            address: "redis://127.0.0.1/".to_string(),
        }
    }
}
