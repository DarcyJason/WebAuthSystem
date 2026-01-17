use crate::domain::user::errors::UserError;
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
        if cmd.password != cmd.confirm_password {
            return Err(ApplicationError::UserError(UserError::PasswordsNotMatch));
        }
        let username = Username::new(cmd.username.clone())?;
        let email = Email::new(cmd.email.clone())?;
        let hash_password = HashPassword::new(cmd.confirm_password.clone())?;
        self.auth_repo
            .register(username, email, hash_password)
            .await?;
        Ok(("register success", ()))
    }
}
