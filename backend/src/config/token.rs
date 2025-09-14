use figment::{Figment, providers::Env};
use serde::{Deserialize, Serialize};

use crate::errors::app_error::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    pub jwt_secret_key: String,
    pub access_expires_in_seconds: i64,
    pub refresh_expires_in_seconds: i64,
}

impl TokenConfig {
    fn figment() -> Figment {
        Figment::new().merge(Env::prefixed(""))
    }
    pub fn init() -> AppResult<Self> {
        Ok(TokenConfig::figment().extract()?)
    }
}
