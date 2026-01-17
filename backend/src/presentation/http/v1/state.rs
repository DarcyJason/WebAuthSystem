use crate::infrastructure::cache::redis::client::RedisClient;
use crate::infrastructure::persistence::surreal::client::SurrealClient;
use crate::presentation::http::v1::config::Config;
use resend_rs::Resend;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub resend: Resend,
    pub surreal: SurrealClient,
    pub redis: RedisClient,
}

impl AppState {
    pub fn new(config: Config, resend: Resend, surreal: SurrealClient, redis: RedisClient) -> Self {
        AppState {
            config,
            resend,
            surreal,
            redis,
        }
    }
}
