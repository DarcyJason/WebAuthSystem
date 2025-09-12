use figment::{Figment, providers::Env};
use serde::{Deserialize, Serialize};

use crate::errors::app_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealDBServerConfig {
    pub surrealdb_host: String,
    pub surrealdb_root_name: String,
    pub surrealdb_root_password: String,
    pub surrealdb_namespace: String,
    pub surrealdb_database: String,
}

impl SurrealDBServerConfig {
    fn figment() -> Figment {
        Figment::new().merge(Env::prefixed(""))
    }
    pub fn init() -> AppResult<Self> {
        Ok(SurrealDBServerConfig::figment().extract()?)
    }
}
