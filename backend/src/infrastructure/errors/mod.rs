pub mod email_verification_token_repository_error;
pub mod user_repository_error;

use crate::infrastructure::errors::email_verification_token_repository_error::EmailVerificationTokenRepositoryError;
use crate::infrastructure::errors::user_repository_error::UserRepositoryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfraError {
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
    #[error(transparent)]
    EmailVerificationTokenRepositoryError(#[from] EmailVerificationTokenRepositoryError),
}
