use thiserror::Error;

use crate::{
    application::commands::auth::register::RegisterCommand,
    domain::user::value_objects::{email::Email, hash_password::HashPassword, useranme::Username},
    presentation::http::v1::{
        errors::ApiError, handlers::auth::register::payload::RegisterPaylaod,
    },
};

#[derive(Debug, Error)]
pub enum RegisterRequestError {
    #[error("username is required")]
    UsernameRequired,
    #[error("username is too long")]
    UsernameTooLong,
    #[error("email is required")]
    EmailRequired,
    #[error("password is required")]
    PasswordRequired,
    #[error("password is too short")]
    PasswordTooShort,
    #[error("password is too long")]
    PasswordTooLong,
    #[error("confirm_password is required")]
    ConfirmPasswordRequired,
    #[error("confirm_password is too short")]
    ConfirmPasswordTooShort,
    #[error("confirm_password is too long")]
    ConfirmPasswordTooLong,
    #[error("username is invalid")]
    UsernameInvalid,
    #[error("email is invalid")]
    EmailInvalid,
    #[error("password is invalid")]
    PasswordInvalid,
    #[error("passwords are not matched")]
    PasswordsNotMatched,
}

impl From<RegisterRequestError> for ApiError {
    fn from(err: RegisterRequestError) -> Self {
        match err {
            RegisterRequestError::UsernameRequired => ApiError::BadRequest {
                message: err.to_string(),
            },
            RegisterRequestError::UsernameTooLong => ApiError::BadRequest {
                message: err.to_string(),
            },
            RegisterRequestError::EmailRequired => ApiError::BadRequest {
                message: err.to_string(),
            },
            RegisterRequestError::PasswordRequired => ApiError::BadRequest {
                message: err.to_string(),
            },
            RegisterRequestError::PasswordTooShort => ApiError::BadRequest {
                message: err.to_string(),
            },
            RegisterRequestError::PasswordTooLong => ApiError::BadRequest {
                message: err.to_string(),
            },
            RegisterRequestError::ConfirmPasswordRequired => ApiError::BadRequest {
                message: err.to_string(),
            },
            RegisterRequestError::ConfirmPasswordTooShort => ApiError::BadRequest {
                message: err.to_string(),
            },
            RegisterRequestError::ConfirmPasswordTooLong => ApiError::BadRequest {
                message: err.to_string(),
            },
            RegisterRequestError::UsernameInvalid => ApiError::BadRequest {
                message: err.to_string(),
            },
            RegisterRequestError::EmailInvalid => ApiError::BadRequest {
                message: err.to_string(),
            },
            RegisterRequestError::PasswordInvalid => ApiError::BadRequest {
                message: err.to_string(),
            },
            RegisterRequestError::PasswordsNotMatched => ApiError::BadRequest {
                message: err.to_string(),
            },
        }
    }
}

impl TryFrom<RegisterPaylaod> for RegisterCommand {
    type Error = RegisterRequestError;
    fn try_from(payload: RegisterPaylaod) -> Result<Self, Self::Error> {
        if payload.username.is_empty() {
            return Err(RegisterRequestError::UsernameRequired);
        }
        if payload.username.len() > 20 {
            return Err(RegisterRequestError::UsernameTooLong);
        }
        if payload.email.is_empty() {
            return Err(RegisterRequestError::EmailRequired);
        }
        if payload.password.is_empty() {
            return Err(RegisterRequestError::PasswordRequired);
        }
        if payload.password.len() < 8 {
            return Err(RegisterRequestError::PasswordTooShort);
        }
        if payload.password.len() > 20 {
            return Err(RegisterRequestError::PasswordTooLong);
        }
        if payload.confirm_password.is_empty() {
            return Err(RegisterRequestError::ConfirmPasswordRequired);
        }
        if payload.confirm_password.len() < 8 {
            return Err(RegisterRequestError::ConfirmPasswordTooShort);
        }
        if payload.confirm_password.len() > 20 {
            return Err(RegisterRequestError::ConfirmPasswordTooLong);
        }
        if payload.password != payload.confirm_password {
            return Err(RegisterRequestError::PasswordsNotMatched);
        }
        Ok(RegisterCommand {
            username: Username::new(payload.username)
                .map_err(|_| RegisterRequestError::UsernameInvalid)?,
            email: Email::new(payload.email).map_err(|_| RegisterRequestError::EmailInvalid)?,
            hash_password: HashPassword::new(payload.password)
                .map_err(|_| RegisterRequestError::PasswordInvalid)?,
        })
    }
}
