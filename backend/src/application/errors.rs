use crate::domain::errors::DomainError;
use crate::infrastructure::errors::InfraError;
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

impl From<DomainError> for CaseError {
    fn from(e: DomainError) -> CaseError {
        match e {
            DomainError::UserDomainError(_) | DomainError::AuthDomainError(_) => {
                CaseError::InternalServerError
            }
        }
    }
}

impl From<InfraError> for CaseError {
    fn from(e: InfraError) -> CaseError {
        match e {
            InfraError::UserRepositoryError(_)
            | InfraError::EmailVerificationTokenRepositoryError(_)
            | InfraError::MailServiceError(_)
            | InfraError::PasswordServiceError(_)
            | InfraError::AccessTokenServiceError(_)
            | InfraError::RefreshTokenServiceError(_) => CaseError::InternalServerError,
        }
    }
}
