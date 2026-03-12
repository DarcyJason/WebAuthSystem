use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("storage is unavailable")]
    StorageUnavailable,
    #[error("persistence operation failed")]
    PersistenceFailed,
    #[error("failed to deserialize stored user data")]
    DeserializationFailed,
}
