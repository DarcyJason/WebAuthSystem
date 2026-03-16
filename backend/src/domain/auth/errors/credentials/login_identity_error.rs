use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoginIdentityError {
    #[error("Invalid user name")]
    UserNameInvalid,
    #[error("Invalid user email")]
    UserEmailInvalid,
    #[error("LoginIdentity required")]
    LoginIdentityRequired,
}
