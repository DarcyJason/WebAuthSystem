use thiserror::Error;

use crate::{
    application::{
        commands::auth::register_command::RegisterCommand,
        results::commands_results::auth::register_result::RegisterResult,
    },
    domain::auth::value_objects::plain_password::{PlainPassword, PlainPasswordError},
    domain::user::value_objects::{
        user_email::{UserEmail, UserEmailError},
        user_name::{UserName, UserNameError},
    },
    presentation::http::v1::{
        errors::api_error::ApiError,
        handlers::auth::register_handler::{
            request::RegisterRequestPayload, response::RegisterResponseData,
        },
    },
};

#[derive(Debug, Error)]
pub enum RegisterRequestPayloadError {
    #[error("username is required")]
    UserNameRequired,
    #[error("username is too long")]
    UserNameTooLong,
    #[error("username is invalid")]
    UserNameInvalid,
    #[error("email is required")]
    UserEmailRequired,
    #[error("email is invalid")]
    UserEmailInvalid,
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
        let name = UserName::new(payload.name).map_err(|e| match e {
            UserNameError::UserNameInvalid => RegisterRequestPayloadError::UserNameInvalid,
            UserNameError::UserNameTooLong => RegisterRequestPayloadError::UserNameTooLong,
        })?;
        let email = UserEmail::new(payload.email).map_err(|e| match e {
            UserEmailError::UserEmailRequired => RegisterRequestPayloadError::UserEmailRequired,
            UserEmailError::UserEmailInvalid => RegisterRequestPayloadError::UserEmailInvalid,
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
            name,
            email,
            plain_password: password,
        })
    }
}

impl From<RegisterResult> for RegisterResponseData {
    fn from(result: RegisterResult) -> Self {
        RegisterResponseData {
            email: result.user_email.value().to_owned(),
        }
    }
}
