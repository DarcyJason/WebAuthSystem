use crate::{
    application::{
        commands::auth::register::RegisterCommand, errors::ApplicationError,
        queries::auth::register::RegisterResult,
    },
    domain::{
        auth::{errors::AuthError, repositories::AuthRepository},
        error::DomainError,
    },
};

#[derive(Debug, Clone)]
pub struct RegisterCase<R>
where
    R: AuthRepository,
{
    auth_repo: R,
}

impl<R> RegisterCase<R>
where
    R: AuthRepository,
{
    pub fn new(auth_repo: R) -> Self {
        RegisterCase { auth_repo }
    }
    pub async fn execute(&self, cmd: RegisterCommand) -> Result<RegisterResult, ApplicationError> {
        let user = self
            .auth_repo
            .register(cmd.username, cmd.email, cmd.hash_password)
            .await
            .map_err(|e| match e {
                DomainError::AuthError(AuthError::UserAlreadyExists) => {
                    ApplicationError::UserAlreadyExists
                }
                _ => ApplicationError::InfrastructureError,
            })?
            .ok_or(ApplicationError::InvalidCredentials)?;
        Ok(RegisterResult::from(user))
    }
}
