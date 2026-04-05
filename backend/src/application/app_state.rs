use crate::application::error::{ApplicationResult, PostgresSnafu, RedisSnafu};
use crate::infrastructure::caches::moka::client::MokaClient;
use crate::infrastructure::caches::moka::user_repository::MokaUserRepository;
use crate::infrastructure::caches::redis::client::RedisClient;
use crate::infrastructure::caches::redis::user_repository::RedisUserRepository;
use crate::infrastructure::config::Config;
use crate::infrastructure::layered::user_repository::LayeredUserRepository;
use crate::infrastructure::persistence::postgres::client::PostgresClient;
use crate::infrastructure::persistence::postgres::user_repository::PostgresUserRepository;
use crate::infrastructure::security::password::Argon2PasswordService;
use resend_rs::Resend;
use snafu::ResultExt;

pub struct AppState {
    pub user_repo: LayeredUserRepository,
    pub password_service: Argon2PasswordService,
}

impl AppState {
    pub async fn new(config: Config) -> ApplicationResult<Self> {
        // initialize infrastructure clients in parallel
        let moka_client = MokaClient::new();
        let _resend_client = Resend::new(&config.resend.api_key);
        let (redis_client, postgres_client) = tokio::join!(
            RedisClient::new(&config.redis),
            PostgresClient::new(&config.postgres)
        );
        let redis_client = redis_client.context(RedisSnafu)?;
        let postgres_client = postgres_client.context(PostgresSnafu)?;

        // User Repository
        let l1_user_repo: MokaUserRepository = MokaUserRepository::new(moka_client.clone());
        let l2_user_repo: RedisUserRepository = RedisUserRepository::new(redis_client.clone());
        let l3_user_repo: PostgresUserRepository =
            PostgresUserRepository::new(postgres_client.clone());
        let user_repo = LayeredUserRepository::new(l1_user_repo, l2_user_repo, l3_user_repo);

        // Password Service
        let password_service = Argon2PasswordService::new();

        Ok(AppState {
            user_repo,
            password_service,
        })
    }
}
