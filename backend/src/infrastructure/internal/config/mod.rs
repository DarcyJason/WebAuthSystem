pub mod jwt_config;
pub mod postgres_config;
pub mod redis_config;
pub mod resend_config;
pub mod server_config;

use figment2::{
    Figment,
    providers::{Env, Serialized},
};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::infrastructure::error::{ConfigSnafu, InfrastructureResult};
use crate::infrastructure::internal::config::{
    jwt_config::JwtConfig, postgres_config::PostgresConfig, redis_config::RedisConfig,
    resend_config::ResendConfig, server_config::ServerConfig,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub resend: ResendConfig,
    pub postgres: PostgresConfig,
    pub redis: RedisConfig,
}

impl Config {
    pub fn init() -> InfrastructureResult<Self> {
        dotenvy::dotenv().ok();
        Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Env::prefixed("").split("__"))
            .extract()
            .context(ConfigSnafu)
    }
}
