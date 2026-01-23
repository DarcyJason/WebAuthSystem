use crate::domain::{auth::errors::AuthError, user::errors::UserError};

pub type DomainResult<T> = Result<T, DomainError>;

pub enum DomainError {
    UserError(UserError),
    AuthError(AuthError),
    RepositoryError,
}

impl From<UserError> for DomainError {
    fn from(err: UserError) -> Self {
        DomainError::UserError(err)
    }
}

impl From<AuthError> for DomainError {
    fn from(err: AuthError) -> Self {
        DomainError::AuthError(err)
    }
}
