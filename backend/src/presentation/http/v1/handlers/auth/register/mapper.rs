use thiserror::Error;

use crate::application::queries::auth::register::RegisterResult;
use crate::presentation::http::v1::handlers::auth::register::response::RegisterResponseData;
use crate::{
    application::commands::auth::register::RegisterCommand,
    domain::{
        auth::value_objects::plain_password::{PlainPassword, PlainPasswordError},
        user::value_objects::{
            email::{Email, EmailError},
            username::{Username, UsernameError},
        },
    },
    presentation::http::v1::{
        errors::ApiError, handlers::auth::register::request::RegisterRequestPayload,
    },
};

#[derive(Debug, Error)]
pub enum RegisterRequestPayloadError {
    #[error("username is required")]
    UsernameRequired,
    #[error("username is too long")]
    UsernameTooLong,
    #[error("username is invalid")]
    UsernameInvalid,
    #[error("email is required")]
    EmailRequired,
    #[error("email is invalid")]
    EmailInvalid,
    #[error("password is required")]
    PasswordRequired,
    #[error("password is too short")]
    PasswordTooShort,
    #[error("password is too long")]
    PasswordTooLong,
    #[error("password is missing digit")]
    PasswordMissingDigit,
    #[error("password is missing lower case letter")]
    PasswordMissingLowerCase,
    #[error("password is missing upper case letter")]
    PasswordMissingUpperCase,
    #[error("password is missing special symbol")]
    PassowrdMissingSpecial,
    #[error("confirm_password is required")]
    ConfirmPasswordRequired,
    #[error("confirm_password is too short")]
    ConfirmPasswordTooShort,
    #[error("confirm_password is too long")]
    ConfirmPasswordTooLong,
    #[error("confirm password is missing digit")]
    ConfirmPasswordMissingDigit,
    #[error("confirm password is missing lower case letter")]
    ConfirmPasswordMissingLowerCase,
    #[error("confirm password is missing upper case letter")]
    ConfirmPasswordMissingUpperCase,
    #[error("confirm password is missing special symbol")]
    ConfirmPassowrdMissingSpecial,
    #[error("passwords are not matched")]
    PasswordsNotMatched,
}

impl From<RegisterRequestPayloadError> for ApiError {
    fn from(err: RegisterRequestPayloadError) -> Self {
        ApiError::BadRequest {
            message: err.to_string(),
        }
    }
}

impl TryFrom<RegisterRequestPayload> for RegisterCommand {
    type Error = RegisterRequestPayloadError;
    fn try_from(payload: RegisterRequestPayload) -> Result<Self, Self::Error> {
        let username = Username::new(payload.username).map_err(|e| match e {
            UsernameError::UsernameInvalid => RegisterRequestPayloadError::UsernameInvalid,
            UsernameError::UsernameTooLong => RegisterRequestPayloadError::UsernameTooLong,
        })?;
        let email = Email::new(payload.email).map_err(|e| match e {
            EmailError::EmailRequired => RegisterRequestPayloadError::EmailRequired,
            EmailError::EmailInvalid => RegisterRequestPayloadError::EmailInvalid,
        })?;
        let password = PlainPassword::new(payload.password).map_err(|e| match e {
            PlainPasswordError::PasswordRequired => RegisterRequestPayloadError::PasswordRequired,
            PlainPasswordError::PasswordTooShort => RegisterRequestPayloadError::PasswordTooShort,
            PlainPasswordError::PasswordTooLong => RegisterRequestPayloadError::PasswordTooLong,
            PlainPasswordError::PasswordMissingDigit => {
                RegisterRequestPayloadError::PasswordMissingDigit
            }
            PlainPasswordError::PasswordMissingLowerCase => {
                RegisterRequestPayloadError::PasswordMissingLowerCase
            }
            PlainPasswordError::PasswordMissingUpperCase => {
                RegisterRequestPayloadError::PasswordMissingUpperCase
            }
            PlainPasswordError::PasswordMissingSpetial => {
                RegisterRequestPayloadError::PassowrdMissingSpecial
            }
        })?;
        let confirm_password =
            PlainPassword::new(payload.confirm_password).map_err(|e| match e {
                PlainPasswordError::PasswordRequired => {
                    RegisterRequestPayloadError::ConfirmPasswordRequired
                }
                PlainPasswordError::PasswordTooShort => {
                    RegisterRequestPayloadError::ConfirmPasswordTooShort
                }
                PlainPasswordError::PasswordTooLong => {
                    RegisterRequestPayloadError::ConfirmPasswordTooLong
                }
                PlainPasswordError::PasswordMissingDigit => {
                    RegisterRequestPayloadError::ConfirmPasswordMissingDigit
                }
                PlainPasswordError::PasswordMissingLowerCase => {
                    RegisterRequestPayloadError::ConfirmPasswordMissingLowerCase
                }
                PlainPasswordError::PasswordMissingUpperCase => {
                    RegisterRequestPayloadError::ConfirmPasswordMissingUpperCase
                }
                PlainPasswordError::PasswordMissingSpetial => {
                    RegisterRequestPayloadError::ConfirmPassowrdMissingSpecial
                }
            })?;
        if password != confirm_password {
            return Err(RegisterRequestPayloadError::PasswordsNotMatched);
        }
        Ok(RegisterCommand {
            username,
            email,
            password,
        })
    }
}

impl From<RegisterResult> for RegisterResponseData {
    fn from(result: RegisterResult) -> Self {
        RegisterResponseData {
            user_id: result.user_id.to_string(),
            username: result.username.to_string(),
            email: result.email.to_string(),
        }
    }
}
