use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub is_development_mode: bool,
    pub frontend_address: String,
    pub backend_ip: String,
    pub backend_port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            is_development_mode: true,
            frontend_address: "http://localhost:5173".to_string(),
            backend_ip: "0.0.0.0".to_string(),
            backend_port: 7878,
        }
    }
}
