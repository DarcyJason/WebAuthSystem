use crate::infrastructure::persistence::surreal::client::SurrealClient;
use crate::presentation::http::v1::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub surreal: SurrealClient
}

impl AppState {
    pub fn new(config: Config, surreal: SurrealClient) -> Self {
        AppState {
            config,
            surreal
        }
    }
}
