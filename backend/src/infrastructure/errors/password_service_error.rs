use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordServiceError {
    #[error("Hash password error")]
    HashPasswordError,
    #[error("Parse hashed password error")]
    ParseHashedPasswordError,
}
