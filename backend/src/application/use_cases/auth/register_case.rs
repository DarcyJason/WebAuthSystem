use crate::domain::user::value_objects::{Email, HashPassword, Username};
use crate::{
    application::{commands::auth::register::RegisterCommand, errors::ApplicationError},
    domain::auth::repositories::AuthRepository,
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
    pub async fn execute(&self, cmd: RegisterCommand) -> Result<(&str, ()), ApplicationError> {
        let username =
            Username::new(cmd.username.clone()).map_err(ApplicationError::DomainError)?;
        let email = Email::new(cmd.email.clone()).map_err(ApplicationError::DomainError)?;
        let hash_password = HashPassword::new(cmd.confirm_password.clone())
            .map_err(ApplicationError::DomainError)?;
        self.auth_repo
            .register(username, email, hash_password)
            .await?;
        Ok(("register success", ()))
    }
}
