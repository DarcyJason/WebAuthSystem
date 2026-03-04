use thiserror::Error;

use crate::domain::auth::{
    repositories::email_verification_token_repository::EmailVerificationTokenRepositoryError,
    services::{
        mail_service::AuthMailServiceError,
        password_service::AuthPasswordServiceError,
        token_service::{AuthAccessTokenServiceError, AuthRefreshTokenServiceError},
    },
    value_objects::plain_password::PlainPasswordError,
};

#[derive(Debug, Error)]
pub enum AuthDomainError {
    #[error(transparent)]
    PlainPasswordError(#[from] PlainPasswordError),
    #[error(transparent)]
    EmailVerificationTokenRepositoryError(#[from] EmailVerificationTokenRepositoryError),
    #[error(transparent)]
    AuthMailServiceError(#[from] AuthMailServiceError),
    #[error(transparent)]
    AuthPasswordServiceError(#[from] AuthPasswordServiceError),
    #[error(transparent)]
    AuthAccessTokenServiceError(#[from] AuthAccessTokenServiceError),
    #[error(transparent)]
    AuthRefreshTokenServiceError(#[from] AuthRefreshTokenServiceError),
}
