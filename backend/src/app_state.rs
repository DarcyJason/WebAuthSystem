use std::sync::Arc;

use crate::domain::auth::repositories::db::user_repo::UserRepository;
use crate::domain::auth::services::mail_service::AuthMailService;
use crate::domain::auth::services::password_service::AuthPasswordService;
use crate::domain::auth::services::token_service::{
    AuthAccessTokenService, AuthRefreshTokenService,
};
use crate::infrastructure::caches::redis::client::RedisClient;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<dyn UserRepository>,
    pub auth_access_token_service: Arc<dyn AuthAccessTokenService>,
    pub auth_refresh_token_service: Arc<dyn AuthRefreshTokenService>,
    pub auth_password_service: Arc<dyn AuthPasswordService>,
    pub auth_mail_service: Arc<dyn AuthMailService>,
    pub redis_client: RedisClient,
}
