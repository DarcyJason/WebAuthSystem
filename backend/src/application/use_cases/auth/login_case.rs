use crate::application::commands::auth::login::LoginCommand;
use crate::application::errors::ApplicationError;
use crate::application::queries::auth::login::LoginResult;
use crate::domain::auth::repositories::{AuthRepository, AuthTokenRepository};
use crate::domain::user::entities::User;

pub struct LoginCase<R, T>
where
    R: AuthRepository,
    T: AuthTokenRepository,
{
    auth_repo: R,
    token_repo: T,
}

impl<R, T> LoginCase<R, T>
where
    R: AuthRepository,
    T: AuthTokenRepository,
{
    pub fn new(auth_repo: R, token_repo: T) -> Self {
        LoginCase {
            auth_repo,
            token_repo,
        }
    }
    pub async fn execute(&self, cmd: LoginCommand) -> Result<LoginResult, ApplicationError> {
        let user: User = self
            .auth_repo
            .login(cmd.identity)
            .await
            .map_err(|_| ApplicationError::InfrastructureError)?
            .ok_or(ApplicationError::UserNotFound)?;
        let is_matched = user
            .hash_password()
            .verify_password(cmd.password)
            .map_err(|_| ApplicationError::ParseHashedPasswordError)?;
        if !is_matched {
            return Err(ApplicationError::InvalidCredentials);
        }
        let access_token = self
            .token_repo
            .generate_access_token(user.id().to_owned())
            .map_err(|_| ApplicationError::GenerateAccessTokenError)?;
        let refresh_token = self
            .token_repo
            .generate_refresh_token()
            .map_err(|_| ApplicationError::GenerateRefreshTokenError)?;
        Ok(LoginResult::from(user, access_token, refresh_token))
    }
}
