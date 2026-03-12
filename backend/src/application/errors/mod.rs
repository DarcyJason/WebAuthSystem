pub mod helper;

use thiserror::Error;

pub type CaseResult<T> = Result<T, CaseError>;

impl CaseError {
    pub fn status_code(self) -> u16 {
        self as u16
    }
}

#[derive(Debug, Clone, Error)]
pub enum CaseError {
    #[error("User not found")]
    UserNotFound = 1000,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User created failed")]
    UserCreatedFailed,
    #[error("Credentials invalid")]
    CredentialsInvalid,
    #[error("Email not verified")]
    EmailNotVerified,
    #[error("Email verification token not found")]
    EmailVerificationTokenNotFound,
    #[error("Email verification token invalid")]
    EmailVerificationTokenInvalid,
    #[error("Internal server error")]
    InternalServerError = 1500,
}
