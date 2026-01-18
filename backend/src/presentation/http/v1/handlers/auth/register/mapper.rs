use crate::{
    application::{commands::auth::register::RegisterCommand, errors::ApplicationError},
    domain::{
        error::DomainError,
        user::value_objects::{Email, HashPassword, Username},
    },
    presentation::http::v1::handlers::auth::register::payload::RegisterPaylaod,
};

impl TryFrom<RegisterPaylaod> for RegisterCommand {
    type Error = ApplicationError;
    fn try_from(payload: RegisterPaylaod) -> Result<Self, Self::Error> {
        if payload.password != payload.confirm_password {
            return Err(DomainError::Validation("passwords not match".to_string()).into());
        }
        Ok(RegisterCommand {
            username: Username::new(payload.username)?,
            email: Email::new(payload.email)?,
            hash_password: HashPassword::new(payload.password)?,
        })
    }
}
