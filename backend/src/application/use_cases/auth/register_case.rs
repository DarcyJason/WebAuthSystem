use crate::{
    application::{commands::auth::register::RegisterCommand, errors::ApplicationError},
    domain::{auth::repositories::AuthRepository, user::entities::User},
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
    pub async fn execute(&self, cmd: RegisterCommand) -> Result<(), ApplicationError> {
        let user = User::register(cmd.username, cmd.email, cmd.password, cmd.confirm_password)
            .map_err(ApplicationError::from)?;
        let execute_result = self.auth_repo
            .register(&user)
            .await
            .map_err(|e| ApplicationError::RepoitoryUnavailable(e.to_string()))?;
        Ok(execute_result)
    }
}
