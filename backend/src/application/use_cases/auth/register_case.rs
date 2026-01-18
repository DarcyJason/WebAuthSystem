use crate::{
    application::{
        commands::auth::register::RegisterCommand, errors::ApplicationError,
        queries::auth::register::RegisterResult,
    },
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
    pub async fn execute(
        &self,
        cmd: RegisterCommand,
    ) -> Result<(&str, RegisterResult), ApplicationError> {
        let user = self
            .auth_repo
            .register(cmd.username, cmd.email, cmd.hash_password)
            .await?
            .ok_or(ApplicationError::Infrastructure)?;
        Ok(("register success", RegisterResult::from(user)))
    }
}
