use thiserror::Error;

use crate::domain::auth::errors::AuthDomainError;
use crate::domain::user::errors::UserDomainError;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error(transparent)]
    AuthDomainError(#[from] AuthDomainError),
    #[error(transparent)]
    UserDomainError(#[from] UserDomainError),
}
