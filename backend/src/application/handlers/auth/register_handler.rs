use crate::{
    application::{commands::auth::register::RegisterCommand, errors::ApplicationError},
    domain::user::{entities::User, repositories::UserRepository},
};

#[derive(Debug, Clone)]
pub struct RegisterHandler<R>
where
    R: UserRepository,
{
    user_repo: R,
}

impl<R> RegisterHandler<R>
where
    R: UserRepository,
{
    pub fn new(user_repo: R) -> Self {
        RegisterHandler { user_repo }
    }
    pub async fn handle(&self, cmd: RegisterCommand) -> Result<(), ApplicationError> {
        let user = User::register(cmd.username, cmd.email, cmd.password, cmd.confirm_password)
            .map_err(ApplicationError::from)?;
        let user = self
            .user_repo
            .save(&user)
            .await
            .map_err(|e| ApplicationError::RepoitoryUnavailable(e.to_string()))?;
        let handle_result = user.is_some();
        if handle_result {
            Ok(())
        } else {
            Err(ApplicationError::RepoitoryUnavailable("User create failed".to_string()))
        }
    }
}
