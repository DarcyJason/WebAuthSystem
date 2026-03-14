use resend_rs::Resend;
use std::sync::Arc;

use crate::domain::auth::repository::email_verification_token_repository::EmailVerificationTokenRepository;
use crate::domain::auth::repository::user_repository::UserRepository;
use crate::domain::auth::service::access_token_service::AccessTokenService;
use crate::domain::auth::service::mail_service::MailService;
use crate::domain::auth::service::password_service::PasswordService;
use crate::domain::auth::service::refresh_token_service::RefreshTokenService;
use crate::infrastructure::caches::layered::email_verification_token_repository::LayeredEmailVerificationTokenRepository;
use crate::infrastructure::caches::layered::user_repository::LayeredUserRepository;
use crate::infrastructure::caches::moka::client::MokaClient;
use crate::infrastructure::caches::moka::email_verification_token_repository::MokaEmailVerificationTokenRepository;
use crate::infrastructure::caches::moka::user_repository::MokaUserRepository;
use crate::infrastructure::caches::redis::client::RedisClient;
use crate::infrastructure::caches::redis::email_verification_token_repository::RedisEmailVerificationTokenRepository;
use crate::infrastructure::caches::redis::user_repository::RedisUserRepository;
use crate::infrastructure::config::Config;
use crate::infrastructure::mail::MailServiceImplementation;
use crate::infrastructure::persistence::surrealdb::client::SurrealDBClient;
use crate::infrastructure::persistence::surrealdb::email_verification_token_repository::SurrealDBEmailVerificationTokenRepository;
use crate::infrastructure::persistence::surrealdb::user_repository::SurrealDBUserRepository;
use crate::infrastructure::security::password::PasswordServiceImplementation;
use crate::infrastructure::security::tokens::access_token::AccessTokenServiceImplementation;
use crate::infrastructure::security::tokens::refresh_token::RefreshTokenServiceImplementation;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<dyn UserRepository>,
    pub access_token_service: Arc<dyn AccessTokenService>,
    pub refresh_token_service: Arc<dyn RefreshTokenService>,
    pub password_service: Arc<dyn PasswordService>,
    pub mail_service: Arc<dyn MailService>,
    pub email_verification_cache: Arc<dyn EmailVerificationTokenRepository>,
}

impl AppState {
    pub async fn init(config: Config) -> anyhow::Result<Self> {
        let surrealdb_client = SurrealDBClient::new(&config.surrealdb).await?;
        let redis_client = RedisClient::new(&config.redis).await?;
        let mail_client = Resend::new(&config.resend.api_key);
        let system_owner_email = config.resend.system_owner_email;
        let l1_user_repo: Arc<dyn UserRepository> =
            Arc::new(MokaUserRepository::new(MokaClient::new()));
        let l2_user_repo: Arc<dyn UserRepository> =
            Arc::new(RedisUserRepository::new(redis_client.clone()));
        let source_user_repo: Arc<dyn UserRepository> =
            Arc::new(SurrealDBUserRepository::new(surrealdb_client.clone()));
        let user_repo: Arc<dyn UserRepository> = Arc::new(LayeredUserRepository::new(
            l1_user_repo,
            l2_user_repo,
            source_user_repo,
        ));
        let access_token_service: Arc<dyn AccessTokenService> =
            Arc::new(AccessTokenServiceImplementation::new(config.jwt.secret));
        let refresh_token_service: Arc<dyn RefreshTokenService> =
            Arc::new(RefreshTokenServiceImplementation::new());
        let password_service: Arc<dyn PasswordService> =
            Arc::new(PasswordServiceImplementation::new());
        let mail_service: Arc<dyn MailService> = Arc::new(MailServiceImplementation::new(
            mail_client,
            system_owner_email,
        ));
        let l1_email_verification_repo: Arc<dyn EmailVerificationTokenRepository> =
            Arc::new(MokaEmailVerificationTokenRepository::new(MokaClient::new()));
        let l2_email_verification_repo: Arc<dyn EmailVerificationTokenRepository> = Arc::new(
            RedisEmailVerificationTokenRepository::new(redis_client.clone()),
        );
        let source_email_verification_repo: Arc<dyn EmailVerificationTokenRepository> = Arc::new(
            SurrealDBEmailVerificationTokenRepository::new(surrealdb_client),
        );
        let email_verification_cache: Arc<dyn EmailVerificationTokenRepository> =
            Arc::new(LayeredEmailVerificationTokenRepository::new(
                l1_email_verification_repo,
                l2_email_verification_repo,
                source_email_verification_repo,
            ));
        Ok(AppState {
            user_repo,
            access_token_service,
            refresh_token_service,
            password_service,
            mail_service,
            email_verification_cache,
        })
    }
}
