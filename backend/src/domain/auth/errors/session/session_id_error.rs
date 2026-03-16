use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionIdError {
    #[error("get session id from &str error")]
    GetSessionIdFromStrError,
}
