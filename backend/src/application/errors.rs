use crate::domain::errors::DomainError;
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
