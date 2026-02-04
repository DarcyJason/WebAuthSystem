use crate::infrastructure::config::jwt_config::JwtConfig;
use crate::infrastructure::config::redis_config::RedisConfig;
use crate::infrastructure::config::resend_config::ResendConfig;
use crate::infrastructure::config::server_config::ServerConfig;
use crate::infrastructure::config::surrealdb_config::SurrealDBConfig;
use dotenvy::dotenv;
use figment2::Figment;
use figment2::providers::{Env, Serialized};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub resend: ResendConfig,
    pub surrealdb: SurrealDBConfig,
    pub redis: RedisConfig,
}

impl AppConfig {
    pub fn init() -> Result<Self, Box<figment2::Error>> {
        dotenv().ok();
        Figment::new()
            .merge(Serialized::defaults(AppConfig::default()))
            .merge(Env::prefixed(""))
            .extract()
            .map_err(Box::new)
    }
}
