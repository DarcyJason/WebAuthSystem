use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendConfig {
    pub api_key: String,
}

impl Default for ResendConfig {
    fn default() -> Self {
        ResendConfig {
            api_key: "redis://127.0.0.1/".to_string(),
        }
    }
}