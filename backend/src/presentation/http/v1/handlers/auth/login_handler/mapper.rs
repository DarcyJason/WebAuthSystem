use thiserror::Error;

use crate::{
    application::{
        commands::auth::login_command::LoginCommand,
        results::commands_results::auth::login_result::LoginResult,
    },
    domain::auth::value_objects::{
        login_identity::{LoginIdentity, LoginIdentityError},
        plain_password::{PlainPassword, PlainPasswordError},
    },
    presentation::http::v1::{
        errors::api_error::ApiError,
        handlers::auth::login_handler::{
            request::LoginRequestPayload, response::LoginResponseData,
        },
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

impl TryFrom<LoginRequestPayload> for LoginCommand {
    type Error = LoginRequestPayloadError;
    fn try_from(payload: LoginRequestPayload) -> Result<Self, Self::Error> {
        let login_identity =
            LoginIdentity::parse(payload.username_or_email).map_err(|e| match e {
                LoginIdentityError::LoginIdentityRequired => {
                    LoginRequestPayloadError::LoginIdentityRequired
                }
                _ => LoginRequestPayloadError::LoginIdentityInvalid,
            })?;
        let plain_password = PlainPassword::new(payload.password).map_err(|e| match e {
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
        Ok(LoginCommand {
            login_identity,
            plain_password,
        })
    }
}

impl From<LoginRequestPayloadError> for ApiError {
    fn from(err: LoginRequestPayloadError) -> Self {
        ApiError::BadRequest {
            message: err.to_string(),
        }
    }
}

impl From<LoginResult> for LoginResponseData {
    fn from(result: LoginResult) -> Self {
        LoginResponseData {
            email: result.user_email.value().to_owned(),
        }
    }
}
