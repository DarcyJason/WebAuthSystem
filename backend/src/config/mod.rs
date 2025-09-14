use serde::{Deserialize, Serialize};

use crate::{
    config::{surrealdb_server::SurrealDBServerConfig, token::TokenConfig},
    errors::app_error::AppResult,
};

pub mod surrealdb_server;
pub mod token;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub surrealdb_server: SurrealDBServerConfig,
    pub token: TokenConfig,
}

impl Config {
    pub fn init() -> AppResult<Self> {
        Ok(Config {
            surrealdb_server: SurrealDBServerConfig::init()?,
            token: TokenConfig::init()?,
        })
    }
}
