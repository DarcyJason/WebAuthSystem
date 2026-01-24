use crate::domain::auth::errors::AuthError;
use crate::domain::auth::value_objects::access_token::AccessToken;
use crate::domain::auth::value_objects::refresh_token::RefreshToken;
use crate::domain::errors::{DomainError, DomainResult};
use crate::infrastructure::token::claims::AccessClaims;
use crate::infrastructure::token::token_repositoy::TokenRepository;
use async_trait::async_trait;
use std::sync::Arc;
use surrealdb::RecordId;

pub trait AuthTokenRepository: Send + Sync + 'static {
    fn generate_access_token(&self, user_id: RecordId) -> DomainResult<AccessToken>;
    fn generate_refresh_token(&self) -> DomainResult<RefreshToken>;
    fn decode_access_token(&self, token: &str) -> DomainResult<AccessClaims>;
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
            .map_err(|_| DomainError::AuthError(AuthError::GenerateAccessTokenFailed))
    }
    fn generate_refresh_token(&self) -> DomainResult<RefreshToken> {
        self.inner
            .generate_refresh_token()
            .map_err(|_| DomainError::AuthError(AuthError::GenerateRefreshTokenFailed))
    }
    fn decode_access_token(&self, token: &str) -> DomainResult<AccessClaims> {
        self.inner
            .decode_access_token(token)
            .map_err(|_| DomainError::AuthError(AuthError::VerifyAccessTokenFailed))
    }
}
