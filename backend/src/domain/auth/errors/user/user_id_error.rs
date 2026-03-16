use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserIdError {
    #[error("Get user id from &str failed")]
    GetUserIdFromStrFailed,
}
