use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
}

impl Default for JwtConfig {
    fn default() -> Self {
        JwtConfig {
            secret: "your_jwt_secret".to_string(),
        }
    }
}
