use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresConfig {
    pub database_url: String,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        PostgresConfig {
            database_url: "postgres://user:password@localhost/postgres".to_string(),
        }
    }
}
