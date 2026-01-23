use crate::domain::health::errors::HealthError;
use crate::domain::{auth::errors::AuthError, user::errors::UserError};

pub type DomainResult<T> = Result<T, DomainError>;

pub enum DomainError {
    HealthError(HealthError),
    UserError(UserError),
    AuthError(AuthError),
    DBUnavailable,
    CacheUnavailable,
    TokenUnavailable,
}
