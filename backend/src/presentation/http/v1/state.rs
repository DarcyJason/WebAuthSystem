use crate::infrastructure::cache::redis::client::RedisClient;
use crate::infrastructure::persistence::surreal::client::SurrealClient;
use crate::presentation::http::v1::config::AppConfig;
use resend_rs::Resend;

#[derive(Debug, Clone)]
pub struct AppState {
    pub app_config: AppConfig,
    pub resend: Resend,
    pub surreal: SurrealClient,
    pub redis: RedisClient,
}

impl AppState {
    pub fn new(
        app_config: AppConfig,
        resend: Resend,
        surreal: SurrealClient,
        redis: RedisClient,
    ) -> Self {
        AppState {
            app_config,
            resend,
            surreal,
            redis,
        }
    }
}
