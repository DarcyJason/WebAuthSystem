use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserIdError {
    #[error("invalid user id format")]
    InvalidFormat,
}
