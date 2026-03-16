use crate::domain::auth::value_objects::credentials::{
    login_identity::LoginIdentityError, plain_password::PlainPasswordError,
};
use thiserror::Error;

pub type AuthDomainResult<T> = Result<T, AuthDomainError>;

#[derive(Debug, Error)]
pub enum AuthDomainError {
    #[error(transparent)]
    LoginIdentityError(#[from] LoginIdentityError),
    #[error(transparent)]
    PlainPasswordError(#[from] PlainPasswordError),
}
