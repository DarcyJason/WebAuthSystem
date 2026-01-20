use crate::application::commands::auth::login::LoginCommand;
use crate::application::errors::ApplicationError;
use crate::application::queries::auth::login::LoginResult;
use crate::domain::auth::repositories::AuthRepository;
use crate::domain::user::entities::User;

pub struct LoginCase<R>
where
    R: AuthRepository,
{
    auth_repo: R,
}

impl<R> LoginCase<R>
where
    R: AuthRepository,
{
    pub fn new(auth_repo: R) -> Self {
        LoginCase { auth_repo }
    }
    pub async fn execute(&self, cmd: LoginCommand) -> Result<LoginResult, ApplicationError> {
        let user: User = self
            .auth_repo
            .login(cmd.identity)
            .await
            .map_err(|_| ApplicationError::InfrastructureError)?
            .ok_or(ApplicationError::UserNotFound)?;
        let is_matched = !user
            .hash_password()
            .verify_password(cmd.password)
            .map_err(|_| ApplicationError::ParseHashedPasswordError)?;
        if is_matched {
            return Err(ApplicationError::InvalidCredentials);
        }
        Ok(LoginResult::from(user))
    }
}
