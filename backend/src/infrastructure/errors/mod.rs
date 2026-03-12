pub mod access_token_service_error;
pub mod email_verification_token_repository_error;
pub mod mail_service_error;
pub mod password_service_error;
pub mod refresh_token_service_error;
pub mod user_repository_error;

use crate::infrastructure::errors::access_token_service_error::AccessTokenServiceError;
use crate::infrastructure::errors::email_verification_token_repository_error::EmailVerificationTokenRepositoryError;
use crate::infrastructure::errors::mail_service_error::MailServiceError;
use crate::infrastructure::errors::password_service_error::PasswordServiceError;
use crate::infrastructure::errors::refresh_token_service_error::RefreshTokenServiceError;
use crate::infrastructure::errors::user_repository_error::UserRepositoryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfraError {
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
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
