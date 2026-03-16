pub mod credentials;
pub mod device;
pub mod session;
pub mod user;

use thiserror::Error;

use crate::domain::auth::errors::credentials::{
    login_identity_error::LoginIdentityError, plain_password_error::PlainPasswordError,
};

pub type AuthDomainResult<T> = Result<T, AuthDomainError>;

#[derive(Debug, Error)]
pub enum AuthDomainError {
    #[error(transparent)]
    LoginIdentityError(#[from] LoginIdentityError),
    #[error(transparent)]
    PlainPasswordError(#[from] PlainPasswordError),
}
