use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResetPasswordRequestPayloadError {
    #[error("email is required")]
    UserEmailRequired,
    #[error("email is invalid")]
    UserEmailInvalid,
}
