use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionIdError {
    #[error("invalid session id format")]
    InvalidFormat,
}
