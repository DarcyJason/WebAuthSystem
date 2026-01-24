use crate::application::errors::ApplicationResult;
use crate::domain::auth::repositories::db::AuthRepository;
use crate::{
    application::{
        commands::auth::register::RegisterCommand, errors::ApplicationError,
        queries::auth::register::RegisterResult,
    },
    domain::{
        auth::errors::AuthError, errors::DomainError,
        user::value_objects::hash_password::HashPassword,
    },
};
use tracing::error;

#[derive(Debug, Clone)]
pub struct RegisterCase<SA>
where
    SA: AuthRepository,
{
    auth_repo: SA,
}

impl<SA> RegisterCase<SA>
where
    SA: AuthRepository,
{
    pub fn new(auth_repo: SA) -> Self {
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
