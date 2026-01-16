use crate::infrastructure::config::server::ServerConfig;
use crate::infrastructure::config::surreal::SurrealConfig;
use dotenvy::dotenv;
use figment2::Figment;
use figment2::providers::{Env, Serialized};
use serde::{Deserialize, Serialize};
use crate::infrastructure::config::redis::RedisConfig;
use crate::infrastructure::config::resend::ResendConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub server: ServerConfig,
    pub resend: ResendConfig,
    pub surreal: SurrealConfig,
    pub redis: RedisConfig,
}

impl Config {
    pub fn new() -> Result<Self, Box<figment2::Error>> {
        dotenv().ok();
        Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Env::prefixed(""))
            .extract()
            .map_err(Box::new)
    }
}
