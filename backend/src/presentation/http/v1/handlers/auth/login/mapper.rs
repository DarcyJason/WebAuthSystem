use thiserror::Error;

use crate::{
    application::commands::auth::login::LoginCommand,
    domain::auth::value_objects::{login_identity::LoginIdentity, plain_password::PlainPassword},
    presentation::http::v1::{errors::ApiError, handlers::auth::login::request::LoginRequest},
};

#[derive(Debug, Error)]
pub enum LoginRequestError {
    #[error("username or email is required")]
    LoginIdentityRequired,
    #[error("password is required")]
    PasswordRequired,
    #[error("password is too short")]
    PasswordTooShort,
    #[error("password is too long")]
    PasswordTooLong,
    #[error("username or email is invalid")]
    LoginIdentityIsInvalid,
    #[error("password is invalid")]
    PasswordIsInvalid,
}

impl From<LoginRequestError> for ApiError {
    fn from(err: LoginRequestError) -> Self {
        match err {
            LoginRequestError::LoginIdentityRequired => ApiError::BadRequest {
                message: err.to_string(),
            },
            LoginRequestError::PasswordRequired => ApiError::BadRequest {
                message: err.to_string(),
            },
            LoginRequestError::PasswordTooShort => ApiError::BadRequest {
                message: err.to_string(),
            },
            LoginRequestError::PasswordTooLong => ApiError::BadRequest {
                message: err.to_string(),
            },
            LoginRequestError::LoginIdentityIsInvalid => ApiError::BadRequest {
                message: err.to_string(),
            },
            LoginRequestError::PasswordIsInvalid => ApiError::BadRequest {
                message: err.to_string(),
            },
        }
    }
}

impl TryFrom<LoginRequest> for LoginCommand {
    type Error = LoginRequestError;
    fn try_from(request: LoginRequest) -> Result<Self, Self::Error> {
        if request.username_or_email.is_empty() {
            return Err(LoginRequestError::LoginIdentityRequired);
        }
        if request.password.is_empty() {
            return Err(LoginRequestError::PasswordRequired);
        }
        if request.password.len() < 8 {
            return Err(LoginRequestError::PasswordTooShort);
        }
        Ok(LoginCommand {
            identity: LoginIdentity::parse(request.username_or_email)
                .map_err(|_| LoginRequestError::LoginIdentityIsInvalid)?,
            password: PlainPassword::new(request.password)
                .map_err(|_| LoginRequestError::PasswordIsInvalid)?,
        })
    }
}
