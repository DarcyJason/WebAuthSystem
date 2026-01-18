use crate::{
    application::{commands::auth::login::LoginCommand, errors::ApplicationError},
    domain::auth::value_objects::{LoginIdentity, PlainPassword},
    presentation::http::v1::handlers::auth::login::payload::LoginPayload,
};

impl TryFrom<LoginPayload> for LoginCommand {
    type Error = ApplicationError;
    fn try_from(payload: LoginPayload) -> Result<Self, Self::Error> {
        let identity = LoginIdentity::parse(payload.username_or_email)?;
        let password = PlainPassword::new(payload.password)?;
        Ok(LoginCommand { identity, password })
    }
}
