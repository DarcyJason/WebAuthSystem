use crate::domain::health::errors::HealthError;
use crate::domain::{auth::errors::AuthError, user::errors::UserError};
use std::fmt::Display;

pub type DomainResult<T> = Result<T, DomainError>;

pub enum DomainError {
    HealthError(HealthError),
    UserError(UserError),
    AuthError(AuthError),
    DBUnavailable,
    CacheUnavailable,
    TokenUnavailable,
}

impl Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::HealthError(err) => write!(f, "Health error: {}", err),
            DomainError::UserError(err) => write!(f, "User error: {}", err),
            DomainError::AuthError(err) => write!(f, "Auth error: {}", err),
            DomainError::DBUnavailable => write!(f, "Database unavailable"),
            DomainError::CacheUnavailable => write!(f, "Cache unavailable"),
            DomainError::TokenUnavailable => write!(f, "Token unavailable"),
        }
    }
}
