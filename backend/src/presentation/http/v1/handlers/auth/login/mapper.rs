use thiserror::Error;

use crate::application::queries::auth::login::LoginResult;
use crate::presentation::http::v1::handlers::auth::login::response::LoginResponseData;
use crate::{
    application::commands::auth::login::LoginCommand,
    domain::auth::value_objects::{
        login_identity::{LoginIdentity, LoginIdentityError},
        plain_password::{PlainPassword, PlainPasswordError},
    },
    presentation::http::v1::{
        errors::ApiError, handlers::auth::login::request::LoginRequestPayload,
    },
};

#[derive(Debug, Error)]
pub enum LoginRequestPayloadError {
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

impl From<LoginRequestPayloadError> for ApiError {
    fn from(err: LoginRequestPayloadError) -> Self {
        ApiError::BadRequest {
            message: err.to_string(),
        }
    }
}

impl TryFrom<LoginRequestPayload> for LoginCommand {
    type Error = LoginRequestPayloadError;
    fn try_from(payload: LoginRequestPayload) -> Result<Self, Self::Error> {
        let identity = LoginIdentity::parse(payload.username_or_email).map_err(|e| match e {
            LoginIdentityError::LoginIdentityRequired => {
                LoginRequestPayloadError::LoginIdentityRequired
            }
            _ => LoginRequestPayloadError::LoginIdentityInvalid,
        })?;
        let password = PlainPassword::new(payload.password).map_err(|e| match e {
            PlainPasswordError::PasswordRequired => LoginRequestPayloadError::PasswordRequired,
            PlainPasswordError::PasswordTooShort => LoginRequestPayloadError::PasswordTooShort,
            PlainPasswordError::PasswordTooLong => LoginRequestPayloadError::PasswordTooLong,
            PlainPasswordError::PasswordMissingDigit => {
                LoginRequestPayloadError::PasswordMissingDigit
            }
            PlainPasswordError::PasswordMissingLowerCase => {
                LoginRequestPayloadError::PasswordMissingLowercase
            }
            PlainPasswordError::PasswordMissingUpperCase => {
                LoginRequestPayloadError::PasswordMissingUpperCase
            }
            PlainPasswordError::PasswordMissingSpetial => {
                LoginRequestPayloadError::PasswordMissingSpecial
            }
        })?;
        Ok(LoginCommand { identity, password })
    }
}

impl From<LoginResult> for LoginResponseData {
    fn from(result: LoginResult) -> Self {
        LoginResponseData {
            user_id: result.user_id.to_string(),
            username: result.username.to_string(),
            email: result.email.to_string(),
        }
    }
}
