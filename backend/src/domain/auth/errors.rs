use crate::domain::auth::services::mail_service::MailServiceError;
use crate::domain::auth::services::password_service::PasswordServiceError;
use crate::domain::auth::services::token_service::{
    AccessTokenServiceError, RefreshTokenServiceError,
};
use crate::domain::auth::value_objects::plain_password::PlainPasswordError;
use crate::infrastructure::errors::email_verification_token_repository_error::EmailVerificationTokenRepositoryError;
use thiserror::Error;

pub type AuthDomainResult<T> = Result<T, AuthDomainError>;

#[derive(Debug, Error)]
pub enum AuthDomainError {
    #[error(transparent)]
    PlainPasswordError(#[from] PlainPasswordError),
    #[error(transparent)]
    EmailVerificationTokenRepositoryError(#[from] EmailVerificationTokenRepositoryError),
    #[error(transparent)]
    MailServiceError(#[from] MailServiceError),
    #[error(transparent)]
    PasswordServiceError(#[from] PasswordServiceError),
    #[error(transparent)]
    AccessTokenServiceError(#[from] AccessTokenServiceError),
    #[error(transparent)]
    RefreshTokenServiceError(#[from] RefreshTokenServiceError),
}
