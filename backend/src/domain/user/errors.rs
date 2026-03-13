use crate::domain::user::entities::user::{user_email::UserEmailError, user_id::UserIdError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserDomainError {
    #[error(transparent)]
    UserIdError(#[from] UserIdError),
    #[error(transparent)]
    UserEmailError(#[from] UserEmailError),
}
