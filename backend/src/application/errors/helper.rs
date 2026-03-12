use crate::application::errors::CaseError;
use crate::domain::errors::DomainError;
use crate::infrastructure::errors::InfraError;

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
