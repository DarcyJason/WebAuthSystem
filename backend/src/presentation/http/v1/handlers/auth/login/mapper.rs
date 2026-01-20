use thiserror::Error;

use crate::{
    application::commands::auth::login::LoginCommand,
    domain::auth::value_objects::{
        login_identity::{LoginIdentity, LoginIdentityError},
        plain_password::{PlainPassword, PlainPasswordError},
    },
    presentation::http::v1::{errors::ApiError, handlers::auth::login::request::LoginRequest},
};

#[derive(Debug, Error)]
pub enum LoginRequestError {
    #[error("username or email is required")]
    LoginIdentityRequired,
    #[error("username or email is invalid")]
    LoginIdentityInvalid,
    #[error("password is required")]
    PasswordRequired,
    #[error("password is too short")]
    PasswordTooShort,
    #[error("password is too long")]
    PasswordTooLong,
    #[error("password is missing digit")]
    PasswordMissingDigit,
    #[error("password is missing lowercase letter")]
    PasswordMissingLowercase,
    #[error("password is missing uppercase letter")]
    PasswordMissingUpperCase,
    #[error("password is missing special symbols")]
    PasswordMissingSpecial,
}

impl From<LoginRequestError> for ApiError {
    fn from(err: LoginRequestError) -> Self {
        match err {
            _ => ApiError::BadRequest {
                message: err.to_string(),
            },
        }
    }
}

impl TryFrom<LoginRequest> for LoginCommand {
    type Error = LoginRequestError;
    fn try_from(request: LoginRequest) -> Result<Self, Self::Error> {
        let identity = LoginIdentity::parse(request.username_or_email).map_err(|e| match e {
            LoginIdentityError::LoginIdentityRequired => LoginRequestError::LoginIdentityRequired,
            _ => LoginRequestError::LoginIdentityInvalid,
        })?;
        let password = PlainPassword::new(request.password).map_err(|e| match e {
            PlainPasswordError::PasswordRequired => LoginRequestError::PasswordRequired,
            PlainPasswordError::PasswordTooShort => LoginRequestError::PasswordTooShort,
            PlainPasswordError::PasswordTooLong => LoginRequestError::PasswordTooLong,
            PlainPasswordError::PasswordMissingDigit => LoginRequestError::PasswordMissingDigit,
            PlainPasswordError::PasswordMissingLowerCase => {
                LoginRequestError::PasswordMissingLowercase
            }
            PlainPasswordError::PasswordMissingUpperCase => {
                LoginRequestError::PasswordMissingUpperCase
            }
            PlainPasswordError::PasswordMissingSpetial => LoginRequestError::PasswordMissingSpecial,
        })?;
        Ok(LoginCommand { identity, password })
    }
}
