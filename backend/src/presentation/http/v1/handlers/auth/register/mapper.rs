use thiserror::Error;

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
        errors::ApiError, handlers::auth::register::request::RegisterRequest,
    },
};

#[derive(Debug, Error)]
pub enum RegisterRequestError {
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

impl From<RegisterRequestError> for ApiError {
    fn from(err: RegisterRequestError) -> Self {
        match err {
            _ => ApiError::BadRequest {
                message: err.to_string(),
            },
        }
    }
}

impl TryFrom<RegisterRequest> for RegisterCommand {
    type Error = RegisterRequestError;
    fn try_from(request: RegisterRequest) -> Result<Self, Self::Error> {
        let username = Username::new(request.username).map_err(|e| match e {
            UsernameError::UsernameInvalid => RegisterRequestError::UsernameInvalid,
            UsernameError::UsernameTooLong => RegisterRequestError::UsernameTooLong,
        })?;
        let email = Email::new(request.email).map_err(|e| match e {
            EmailError::EmailRequired => RegisterRequestError::EmailRequired,
            EmailError::EmailInvalid => RegisterRequestError::EmailInvalid,
        })?;
        let password = PlainPassword::new(request.password).map_err(|e| match e {
            PlainPasswordError::PasswordRequired => RegisterRequestError::PasswordRequired,
            PlainPasswordError::PasswordTooShort => RegisterRequestError::PasswordTooShort,
            PlainPasswordError::PasswordTooLong => RegisterRequestError::PasswordTooLong,
            PlainPasswordError::PasswordMissingDigit => RegisterRequestError::PasswordMissingDigit,
            PlainPasswordError::PasswordMissingLowerCase => {
                RegisterRequestError::PasswordMissingLowerCase
            }
            PlainPasswordError::PasswordMissingUpperCase => {
                RegisterRequestError::PasswordMissingUpperCase
            }
            PlainPasswordError::PasswordMissingSpetial => {
                RegisterRequestError::PassowrdMissingSpecial
            }
        })?;
        let confirm_password =
            PlainPassword::new(request.confirm_password).map_err(|e| match e {
                PlainPasswordError::PasswordRequired => {
                    RegisterRequestError::ConfirmPasswordRequired
                }
                PlainPasswordError::PasswordTooShort => {
                    RegisterRequestError::ConfirmPasswordTooShort
                }
                PlainPasswordError::PasswordTooLong => RegisterRequestError::ConfirmPasswordTooLong,
                PlainPasswordError::PasswordMissingDigit => {
                    RegisterRequestError::ConfirmPasswordMissingDigit
                }
                PlainPasswordError::PasswordMissingLowerCase => {
                    RegisterRequestError::ConfirmPasswordMissingLowerCase
                }
                PlainPasswordError::PasswordMissingUpperCase => {
                    RegisterRequestError::ConfirmPasswordMissingUpperCase
                }
                PlainPasswordError::PasswordMissingSpetial => {
                    RegisterRequestError::ConfirmPassowrdMissingSpecial
                }
            })?;
        if password != confirm_password {
            return Err(RegisterRequestError::PasswordsNotMatched);
        }
        Ok(RegisterCommand {
            username,
            email,
            password,
        })
    }
}
