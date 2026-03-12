use crate::domain::auth::errors::AuthDomainError;
use crate::domain::user::errors::UserDomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error(transparent)]
    UserDomainError(#[from] UserDomainError),
    #[error(transparent)]
    AuthDomainError(#[from] AuthDomainError),
}
