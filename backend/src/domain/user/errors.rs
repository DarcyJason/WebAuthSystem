use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("create user failed")]
    CreateUserFailed,
    #[error("user not found")]
    UserNotFound,
}
