use thiserror::Error;

use crate::{
    application::commands::auth::register::RegisterCommand,
    domain::user::value_objects::{email::Email, hash_password::HashPassword, username::Username},
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

impl TryFrom<RegisterRequest> for RegisterCommand {
    type Error = RegisterRequestError;
    fn try_from(request: RegisterRequest) -> Result<Self, Self::Error> {
        if request.username.is_empty() {
            return Err(RegisterRequestError::UsernameRequired);
        }
        if request.username.len() > 20 {
            return Err(RegisterRequestError::UsernameTooLong);
        }
        if request.email.is_empty() {
            return Err(RegisterRequestError::EmailRequired);
        }
        if request.password.is_empty() {
            return Err(RegisterRequestError::PasswordRequired);
        }
        if request.password.len() < 8 {
            return Err(RegisterRequestError::PasswordTooShort);
        }
        if request.password.len() > 20 {
            return Err(RegisterRequestError::PasswordTooLong);
        }
        if request.confirm_password.is_empty() {
            return Err(RegisterRequestError::ConfirmPasswordRequired);
        }
        if request.confirm_password.len() < 8 {
            return Err(RegisterRequestError::ConfirmPasswordTooShort);
        }
        if request.confirm_password.len() > 20 {
            return Err(RegisterRequestError::ConfirmPasswordTooLong);
        }
        if request.password != request.confirm_password {
            return Err(RegisterRequestError::PasswordsNotMatched);
        }
        Ok(RegisterCommand {
            username: Username::new(request.username)
                .map_err(|_| RegisterRequestError::UsernameInvalid)?,
            email: Email::new(request.email).map_err(|_| RegisterRequestError::EmailInvalid)?,
            hash_password: HashPassword::new(request.password)
                .map_err(|_| RegisterRequestError::PasswordInvalid)?,
        })
    }
}
