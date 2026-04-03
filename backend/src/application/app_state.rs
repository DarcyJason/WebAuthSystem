use crate::application::error::{ApplicationResult, PostgresSnafu, RedisSnafu};
use crate::domain::identities::repositories::user_repository::UserRepository;
use crate::infrastructure::caches::moka::client::MokaClient;
use crate::infrastructure::caches::moka::user_repository::MokaUserRepository;
use crate::infrastructure::caches::redis::client::RedisClient;
use crate::infrastructure::caches::redis::user_repository::RedisUserRepository;
use crate::infrastructure::config::Config;
use crate::infrastructure::persistence::postgres::client::PostgresClient;
use crate::infrastructure::persistence::postgres::user_repository::PostgresUserRepository;
use snafu::ResultExt;
use std::sync::Arc;

pub struct AppState {
    pub user_repo: Arc<dyn UserRepository>,
}

impl AppState {
    pub async fn new(config: Config) -> ApplicationResult<Self> {
        let moka_client = MokaClient::new();
        let redis_client = RedisClient::new(&config.redis).await.context(RedisSnafu)?;
        let postgres_client = PostgresClient::new(&config.postgres)
            .await
            .context(PostgresSnafu)?;
        let l1_user_repo = MokaUserRepository::new(moka_client.clone());
        let l2_user_repo = RedisUserRepository::new(redis_client.clone());
        let l3_user_repo = PostgresUserRepository::new(&postgres_client);
        let user_repo: Arc<dyn UserRepository> = Arc::new(l3_user_repo);
        Ok(AppState { user_repo })
    }
}
