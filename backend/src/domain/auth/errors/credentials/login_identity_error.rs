use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoginIdentityError {
    #[error("Invalid user name")]
    InvalidUserName,
    #[error("Invalid user email")]
    InvalidUserEmail,
    #[error("LoginIdentity required")]
    LoginIdentityRequired,
}
