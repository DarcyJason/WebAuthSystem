use serde::{Deserialize, Serialize};

use crate::{config::surrealdb_server::SurrealDBServerConfig, errors::app_error::AppResult};

pub mod surrealdb_server;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub surrealdb_server: SurrealDBServerConfig,
}

impl Config {
    pub fn init() -> AppResult<Self> {
        Ok(Config {
            surrealdb_server: SurrealDBServerConfig::init()?,
        })
    }
}
