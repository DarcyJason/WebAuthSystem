use crate::domain::health::errors::HealthError;
use crate::domain::{auth::errors::AuthError, user::errors::UserError};
use thiserror::Error;

pub type DomainResult<T> = Result<T, DomainError>;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error(transparent)]
    HealthError(HealthError),
    #[error(transparent)]
    UserError(UserError),
    #[error(transparent)]
    AuthError(AuthError),
    #[error("DB is unavailable")]
    SurrealDBUnavailable,
    #[error("Cache is unavailable")]
    RedisCacheUnavailable,
    #[error("Token is unavailable")]
    TokenUnavailable,
}
