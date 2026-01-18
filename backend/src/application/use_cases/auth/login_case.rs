use crate::application::queries::auth::login::LoginResult;
use crate::application::commands::auth::login::LoginCommand;
use crate::application::errors::ApplicationError;
use crate::domain::auth::errors::AuthError;
use crate::domain::auth::repositories::AuthRepository;

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
    pub async fn execute(
        &self,
        cmd: LoginCommand,
    ) -> Result<(&str, LoginResult), ApplicationError> {
        let user = self
            .auth_repo
            .login(cmd.identity)
            .await?
            .ok_or(ApplicationError::DomainError(AuthError::InvalidCredentials.into()))?;
        user.hash_password().verify_password(&cmd.password)?;
        Ok(("login success", LoginResult::from(user)))
    }
}
