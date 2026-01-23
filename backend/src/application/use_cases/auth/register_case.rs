use crate::{
    application::{
        commands::auth::register::RegisterCommand, errors::ApplicationError,
        queries::auth::register::RegisterResult,
    },
    domain::{
        auth::{ repositories::AuthRepository},
        user::value_objects::hash_password::HashPassword,
    },
};
use crate::infrastructure::errors::InfrastructureError;
use crate::infrastructure::persistence::surreal::errors::SurrealDBError;

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
        let hash_password = HashPassword::new(cmd.password)
            .map_err(|_| ApplicationError::ParseHashedPasswordError)?;
        let user = self
            .auth_repo
            .register(cmd.username, cmd.email, hash_password)
            .await
            .map_err(|e| match e {
                InfrastructureError::SurrealDBError(SurrealDBError::RepositoryError(msg)) if msg == "User already exists" => ApplicationError::UserAlreadyExists,
                _ => ApplicationError::InfrastructureError,
            })?
            .ok_or(ApplicationError::InvalidCredentials)?;
        Ok(RegisterResult::from(user))
    }
}
