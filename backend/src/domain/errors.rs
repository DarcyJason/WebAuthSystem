use thiserror::Error;

use crate::domain::user::errors::UserDomainError;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error(transparent)]
    UserDomainError(#[from] UserDomainError),
}
