use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailVerificationTokenRepositoryError {
    #[error("token store unavailable")]
    TokenStoreUnavailable,
    #[error("token not found")]
    TokenNotFound,
    #[error("token remove failed")]
    TokenRemoveFailed,
}
