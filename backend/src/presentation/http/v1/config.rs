use crate::infrastructure::config::jwt::JwtConfig;
use crate::infrastructure::config::redis::RedisConfig;
use crate::infrastructure::config::resend::ResendConfig;
use crate::infrastructure::config::server::ServerConfig;
use crate::infrastructure::config::surreal::SurrealConfig;
use dotenvy::dotenv;
use figment2::providers::{Env, Serialized};
use figment2::Figment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub resend: ResendConfig,
    pub surreal: SurrealConfig,
    pub redis: RedisConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, Box<figment2::Error>> {
        dotenv().ok();
        Figment::new()
            .merge(Serialized::defaults(AppConfig::default()))
            .merge(Env::prefixed(""))
            .extract()
            .map_err(Box::new)
    }
}
