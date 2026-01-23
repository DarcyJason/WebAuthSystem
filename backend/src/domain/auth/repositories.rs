use crate::domain::auth::errors::AuthError;
use crate::domain::auth::value_objects::access_token::AccessToken;
use crate::domain::auth::value_objects::login_identity::LoginIdentity;
use crate::domain::auth::value_objects::refresh_token::RefreshToken;
use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::user::entities::User;
use crate::domain::user::value_objects::email::Email;
use crate::domain::user::value_objects::hash_password::HashPassword;
use crate::domain::user::value_objects::username::Username;
use crate::infrastructure::errors::InfrastructureError;
use crate::infrastructure::persistence::surreal::auth_repository::SurrealAuthRepository;
use crate::infrastructure::persistence::surreal::errors::SurrealDBError;
use crate::infrastructure::token::token_repositoy::TokenRepository;
use async_trait::async_trait;
use std::sync::Arc;
use surrealdb::RecordId;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn register(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> DomainResult<Option<User>>;
    async fn login(&self, identity: LoginIdentity) -> DomainResult<Option<User>>;
    async fn logout(&self, user_id: &str) -> DomainResult<()>;
    async fn forget_password(&self, email: &str) -> DomainResult<()>;
    async fn reset_password(&self, token: &str, new_password: &str) -> DomainResult<()>;
}

pub struct SurrealAuthRepositoryAdapter {
    inner: SurrealAuthRepository,
}

impl SurrealAuthRepositoryAdapter {
    pub fn new(inner: SurrealAuthRepository) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl AuthRepository for SurrealAuthRepositoryAdapter {
    async fn register(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> DomainResult<Option<User>> {
        self.inner
            .register(username, email, hash_password)
            .await
            .map_err(|e| match e {
                InfrastructureError::SurrealDBError(SurrealDBError::RepositoryError(msg))
                    if msg == "user already exists" =>
                {
                    DomainError::AuthError(AuthError::UserAlreadyExists)
                }
                _ => DomainError::DBUnavailable,
            })
    }
    async fn login(&self, identity: LoginIdentity) -> DomainResult<Option<User>> {
        self.inner.login(identity).await.map_err(|e| match e {
            InfrastructureError::SurrealDBError(SurrealDBError::RepositoryError(msg))
                if msg == "user not found" =>
            {
                DomainError::AuthError(AuthError::UserNotFound)
            }
            _ => DomainError::DBUnavailable,
        })
    }
    async fn logout(&self, _user_id: &str) -> DomainResult<()> {
        todo!()
    }
    async fn forget_password(&self, _email: &str) -> DomainResult<()> {
        todo!()
    }
    async fn reset_password(&self, _token: &str, _new_password: &str) -> DomainResult<()> {
        todo!()
    }
}

pub trait AuthTokenRepository: Send + Sync {
    fn generate_access_token(&self, user_id: RecordId) -> DomainResult<AccessToken>;
    fn generate_refresh_token(&self) -> DomainResult<RefreshToken>;
    fn verify_access_token(&self, token: &str) -> DomainResult<bool>;
}

pub struct AuthTokenRepositoryAdapter {
    inner: Arc<TokenRepository>,
}

impl AuthTokenRepositoryAdapter {
    pub fn new(inner: Arc<TokenRepository>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl AuthTokenRepository for AuthTokenRepositoryAdapter {
    fn generate_access_token(&self, user_id: RecordId) -> DomainResult<AccessToken> {
        self.inner
            .generate_access_token(user_id)
            .map_err(|e| match e {
                _ => DomainError::AuthError(AuthError::GenerateAccessTokenFailed),
            })
    }
    fn generate_refresh_token(&self) -> DomainResult<RefreshToken> {
        self.inner.generate_refresh_token().map_err(|e| match e {
            _ => DomainError::AuthError(AuthError::GenerateRefreshTokenFailed),
        })
    }
    fn verify_access_token(&self, token: &str) -> DomainResult<bool> {
        self.inner.verify_access_token(token).map_err(|e| match e {
            _ => DomainError::AuthError(AuthError::VerifyAccessTokenFailed),
        })
    }
}
