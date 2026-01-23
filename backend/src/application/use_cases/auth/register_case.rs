use crate::application::errors::ApplicationResult;
use crate::{
    application::{
        commands::auth::register::RegisterCommand, errors::ApplicationError,
        queries::auth::register::RegisterResult,
    },
    domain::{
        auth::{errors::AuthError, repositories::AuthRepository},
        errors::DomainError,
        user::value_objects::hash_password::HashPassword,
    },
};
use tracing::error;

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
    pub async fn execute(&self, cmd: RegisterCommand) -> ApplicationResult<RegisterResult> {
        let hash_password = HashPassword::new(cmd.password)
            .map_err(|_| ApplicationError::ParseHashedPasswordError)?;
        let user = self
            .auth_repo
            .register(cmd.username, cmd.email, hash_password)
            .await
            .map_err(|e| match e {
                DomainError::AuthError(AuthError::UserAlreadyExists) => {
                    error!("Handle register failed: user already exists");
                    ApplicationError::UserAlreadyExists
                }
                _ => {
                    error!("Handle register failed: SurrealDB query error");
                    ApplicationError::InfrastructureError
                }
            })?
            .ok_or(ApplicationError::InfrastructureError)?;
        Ok(RegisterResult::from(user))
    }
}
