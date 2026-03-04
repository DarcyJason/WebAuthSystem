use resend_rs::Resend;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::domain::auth::repositories::email_verification_token_repository::EmailVerificationTokenRepository;
use crate::domain::auth::services::mail_service::AuthMailService;
use crate::domain::auth::services::password_service::AuthPasswordService;
use crate::domain::auth::services::token_service::{
    AuthAccessTokenService, AuthRefreshTokenService,
};
use crate::domain::user::repositories::user_repository::UserRepository;
use crate::infrastructure::caches::redis::client::RedisClient;
use crate::infrastructure::caches::redis::email_verification_token_repository::RedisEmailVerificationTokenRepository;
use crate::infrastructure::config::Config;
use crate::infrastructure::mail::MailService;
use crate::infrastructure::persistence::surrealdb::client::SurrealDBClient;
use crate::infrastructure::persistence::surrealdb::user_repository::SurrealDBUserRepository;
use crate::infrastructure::security::password::PasswordService;
use crate::infrastructure::security::tokens::access_token::AccessTokenService;
use crate::infrastructure::security::tokens::refresh_token::RefreshTokenService;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<dyn UserRepository>,
    pub auth_access_token_service: Arc<dyn AuthAccessTokenService>,
    pub auth_refresh_token_service: Arc<dyn AuthRefreshTokenService>,
    pub auth_password_service: Arc<dyn AuthPasswordService>,
    pub auth_mail_service: Arc<dyn AuthMailService>,
    pub email_verification_cache: Arc<Mutex<dyn EmailVerificationTokenRepository>>,
}

impl AppState {
    pub async fn init(config: Config) -> anyhow::Result<Self> {
        let surrealdb_client = SurrealDBClient::new(&config.surrealdb).await?;
        let redis_client = RedisClient::new(&config.redis).await?;
        let mail_client = Resend::new(&config.resend.api_key);
        let user_repo: Arc<dyn UserRepository> =
            Arc::new(SurrealDBUserRepository::new(surrealdb_client));
        let auth_access_token_service: Arc<dyn AuthAccessTokenService> =
            Arc::new(AccessTokenService::new(config.jwt.secret));
        let auth_refresh_token_service: Arc<dyn AuthRefreshTokenService> =
            Arc::new(RefreshTokenService::new());
        let auth_password_service: Arc<dyn AuthPasswordService> = Arc::new(PasswordService::new());
        let auth_mail_service: Arc<dyn AuthMailService> = Arc::new(MailService::new(mail_client));
        let email_verification_cache: Arc<Mutex<dyn EmailVerificationTokenRepository>> = Arc::new(
            Mutex::new(RedisEmailVerificationTokenRepository::new(redis_client)),
        );
        Ok(AppState {
            user_repo,
            auth_access_token_service,
            auth_refresh_token_service,
            auth_password_service,
            auth_mail_service,
            email_verification_cache,
        })
    }
}
