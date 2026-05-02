use crate::application::error::{ApplicationResult, PostgresFailedSnafu, RedisFailedSnafu};
use crate::domain::common::value_objects::time::ttl::TTL;
use crate::infrastructure::external::resend::ResendMailService;
use crate::infrastructure::internal::caches::moka::client::MokaClient;
use crate::infrastructure::internal::caches::moka::refresh_token_repository::MokaRefreshTokenRepository;
use crate::infrastructure::internal::caches::moka::user_repository::MokaUserRepository;
use crate::infrastructure::internal::caches::moka::verification_token_repository::MokaVerificationTokenRepository;
use crate::infrastructure::internal::caches::redis::client::RedisClient;
use crate::infrastructure::internal::caches::redis::refresh_token_repository::RedisRefreshTokenRepository;
use crate::infrastructure::internal::caches::redis::user_repository::RedisUserRepository;
use crate::infrastructure::internal::caches::redis::verification_token_repository::RedisVerificationTokenRepository;
use crate::infrastructure::internal::config::Config;
use crate::infrastructure::internal::layered::refresh_token_repository::LayeredRefreshTokenRepository;
use crate::infrastructure::internal::layered::user_repository::LayeredUserRepository;
use crate::infrastructure::internal::layered::verification_token_repository::LayeredVerificationTokenRepository;
use crate::infrastructure::internal::persistence::postgres::client::PostgresClient;
use crate::infrastructure::internal::persistence::postgres::refresh_token_repository::PostgresRefreshTokenRepository;
use crate::infrastructure::internal::persistence::postgres::user_repository::PostgresUserRepository;
use crate::infrastructure::internal::persistence::postgres::verification_token_repository::PostgresVerificationTokenRepository;
use crate::infrastructure::internal::security::password::Argon2PasswordService;
use crate::infrastructure::internal::security::tokens::access_token::DefaultAccessTokenService;
use crate::infrastructure::internal::security::tokens::refresh_token::DefaultRefreshTokenService;
use crate::infrastructure::internal::security::tokens::verification_token::DefaultVerificationTokenService;
use resend_rs::Resend;
use snafu::ResultExt;

#[derive(Debug, Clone)]
pub struct AppState {
    pub user_repo: LayeredUserRepository,
    pub password_service: Argon2PasswordService,
    pub access_token_service: DefaultAccessTokenService,
    pub refresh_token_service: DefaultRefreshTokenService,
    pub refresh_token_repo: LayeredRefreshTokenRepository,
    pub verification_token_repo: LayeredVerificationTokenRepository,
    pub verification_token_service: DefaultVerificationTokenService,
    pub mail_service: ResendMailService,
}

impl AppState {
    pub async fn new(config: Config) -> ApplicationResult<Self> {
        let moka_client = MokaClient::new();
        let resend_client = Resend::new(&config.resend.api_key);
        let (redis_client, postgres_client) = tokio::try_join!(
            async {
                RedisClient::new(&config.redis)
                    .await
                    .context(RedisFailedSnafu)
            },
            async {
                PostgresClient::new(&config.postgres)
                    .await
                    .context(PostgresFailedSnafu)
            },
        )?;

        
        let user_repo = LayeredUserRepository::new(
            MokaUserRepository::with_ttl(moka_client.clone(), TTL::from_seconds(300)),
            RedisUserRepository::with_ttl(redis_client.clone(), TTL::from_seconds(1800)),
            PostgresUserRepository::new(postgres_client.clone()),
        );

        
        let refresh_token_repo = LayeredRefreshTokenRepository::new(
            MokaRefreshTokenRepository::with_ttl(moka_client.clone(), TTL::from_seconds(300)),
            RedisRefreshTokenRepository::with_ttl(redis_client.clone(), TTL::from_seconds(1800)),
            PostgresRefreshTokenRepository::new(postgres_client.clone()),
        );

        
        let verification_token_repo = LayeredVerificationTokenRepository::new(
            MokaVerificationTokenRepository::with_ttl(moka_client.clone(), TTL::from_seconds(300)),
            RedisVerificationTokenRepository::with_ttl(
                redis_client.clone(),
                TTL::from_seconds(1800),
            ),
            PostgresVerificationTokenRepository::new(postgres_client.clone()),
        );

        let password_service = Argon2PasswordService::new();
        let access_token_service =
            DefaultAccessTokenService::new(&config.jwt.secret, config.jwt.expires_in_seconds);
        let refresh_token_service =
            DefaultRefreshTokenService::new(config.jwt.refresh_token_expires_in_days);
        let verification_token_service = DefaultVerificationTokenService::new(
            config.jwt.email_verify_expires_in_seconds,
            config.jwt.password_reset_expires_in_seconds,
        );
        let mail_service =
            ResendMailService::new(resend_client, config.resend.system_owner_email.clone());

        Ok(AppState {
            user_repo,
            password_service,
            access_token_service,
            refresh_token_service,
            refresh_token_repo,
            verification_token_repo,
            verification_token_service,
            mail_service,
        })
    }
}
