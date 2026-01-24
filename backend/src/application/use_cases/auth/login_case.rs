use crate::application::commands::auth::login::LoginCommand;
use crate::application::errors::{ApplicationError, ApplicationResult};
use crate::application::queries::auth::login::LoginResult;
use crate::domain::auth::errors::AuthError;
use crate::domain::auth::repositories::db::AuthRepository;
use crate::domain::auth::repositories::token::AuthTokenRepository;
use crate::domain::errors::DomainError;
use tracing::error;

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
    pub async fn execute(&self, cmd: LoginCommand) -> ApplicationResult<LoginResult> {
        let user = self
            .auth_repo
            .login(cmd.identity)
            .await
            .map_err(|e| match e {
                DomainError::AuthError(AuthError::UserNotFound) => {
                    error!("Handle login failed: User not found");
                    ApplicationError::UserNotFound
                }
                _ => {
                    error!("Handle login failed: SurrealDB query error");
                    ApplicationError::InfrastructureError
                }
            })?
            .ok_or(ApplicationError::InfrastructureError)?;
        let is_matched = user
            .hash_password()
            .verify_password(cmd.password)
            .map_err(|_| {
                error!("Handle login failed: parse hashed password error");
                ApplicationError::ParseHashedPasswordError
            })?;
        if !is_matched {
            error!("Handle login failed: invalid credentials");
            return Err(ApplicationError::InvalidCredentials);
        }
        let access_token = self
            .token_repo
            .generate_access_token(user.id().to_owned())
            .map_err(|_| {
                error!("Handle login failed: generate access token error");
                ApplicationError::GenerateAccessTokenError
            })?;
        let refresh_token = self.token_repo.generate_refresh_token().map_err(|_| {
            error!("Handle login failed: generate refresh token error");
            ApplicationError::GenerateRefreshTokenError
        })?;
        Ok(LoginResult::from(user, access_token, refresh_token))
    }
}
