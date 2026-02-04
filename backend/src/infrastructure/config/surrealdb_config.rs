use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealDBConfig {
    pub address: String,
    pub root_name: String,
    pub root_password: String,
    pub namespace: String,
    pub database: String,
}

impl Default for SurrealDBConfig {
    fn default() -> Self {
        SurrealDBConfig {
            address: "localhost:10086".to_string(),
            root_name: "root".to_string(),
            root_password: "root".to_string(),
            namespace: "WebAuthSystem".to_string(),
            database: "backend".to_string(),
        }
    }
}
