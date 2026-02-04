use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendConfig {
    pub api_key: String,
}

impl Default for ResendConfig {
    fn default() -> Self {
        ResendConfig {
            api_key: "your_resend_api_key".to_string(),
        }
    }
}
