use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserEmailError {
    #[error("User email is required")]
    UserEmailRequired,
    #[error("User email is invalid")]
    UserEmailInvalid,
}
