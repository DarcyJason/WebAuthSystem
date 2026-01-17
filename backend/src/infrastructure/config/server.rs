use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub is_development_mode: bool,
    pub frontend_address: String,
    pub backend_address: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            is_development_mode: true,
            frontend_address: "http://localhost:5173".to_string(),
            backend_address: "0.0.0.0:7878".to_string(),
        }
    }
}
