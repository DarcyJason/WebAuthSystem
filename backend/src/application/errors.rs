use crate::domain::errors::DomainError;
use crate::domain::{
    auth::errors::AuthDomainError,
    auth::{
        repositories::email_verification_token_repository::EmailVerificationTokenRepositoryError,
        services::{
            mail_service::AuthMailServiceError,
            password_service::AuthPasswordServiceError,
            token_service::{AuthAccessTokenServiceError, AuthRefreshTokenServiceError},
        },
    },
    user::{
        errors::UserDomainError, repositories::user_repository::UserRepositoryError,
        value_objects::user_email::UserEmailError,
    },
};
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    DomainError(#[from] DomainError),
    #[error("Storage error")]
    StorageError,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Create user failed")]
    CreateUserFailed,
    #[error("User not found")]
    UserNotFound,
    #[error("Hash password failed")]
    HashPasswordFailed,
    #[error("Parse hashed password failed")]
    ParseHashedPasswordFailed,
    #[error("Credentials are invalid")]
    CredentialsInvalid,
    #[error("Email not verified")]
    EmailNotVerified,
    #[error("Encode access_token failed")]
    EncodeAccessTokenFailed,
    #[error("Decode access_token failed")]
    DecodeAccessTokenFailed,
    #[error("Generate refresh_token failed")]
    GenerateRefreshTokenFailed,
    #[error("System owner email is invalid")]
    SystemOwnerEmailInvalid,
    #[error("Send verification email failed")]
    SendVerificationEmailFailed,
    #[error("Save email verification token failed")]
    SaveEmailVerificationTokenFailed,
    #[error("Get email verification token failed")]
    GetEmailVerificationTokenFailed,
    #[error("Email verifiction token not found")]
    EmailVerificationTokenNotFound,
    #[error("Email verification token is invalid")]
    EmailVerificationTokenInvalid,
}

impl From<UserRepositoryError> for AppError {
    fn from(value: UserRepositoryError) -> Self {
        AppError::DomainError(DomainError::from(UserDomainError::from(value)))
    }
}

impl From<AuthPasswordServiceError> for AppError {
    fn from(value: AuthPasswordServiceError) -> Self {
        AppError::DomainError(DomainError::from(AuthDomainError::from(value)))
    }
}

impl From<AuthAccessTokenServiceError> for AppError {
    fn from(value: AuthAccessTokenServiceError) -> Self {
        AppError::DomainError(DomainError::from(AuthDomainError::from(value)))
    }
}

impl From<AuthRefreshTokenServiceError> for AppError {
    fn from(value: AuthRefreshTokenServiceError) -> Self {
        AppError::DomainError(DomainError::from(AuthDomainError::from(value)))
    }
}

impl From<EmailVerificationTokenRepositoryError> for AppError {
    fn from(value: EmailVerificationTokenRepositoryError) -> Self {
        AppError::DomainError(DomainError::from(AuthDomainError::from(value)))
    }
}

impl From<AuthMailServiceError> for AppError {
    fn from(value: AuthMailServiceError) -> Self {
        AppError::DomainError(DomainError::from(AuthDomainError::from(value)))
    }
}

impl From<UserEmailError> for AppError {
    fn from(value: UserEmailError) -> Self {
        AppError::DomainError(DomainError::from(UserDomainError::from(value)))
    }
}
