use std::sync::Arc;

use crate::application::auth::commands::login_command::LoginCommand;
use crate::application::auth::results::login_result::LoginResult;
use crate::application::errors::{CaseError, CaseResult};
use crate::domain::auth::entity::user::User;
use crate::domain::auth::repository::user_repository::UserRepository;
use crate::domain::auth::service::access_token_service::AccessTokenService;
use crate::domain::auth::service::password_service::PasswordService;
use crate::domain::auth::service::refresh_token_service::RefreshTokenService;
use crate::domain::auth::value_objects::credentials::login_identity::LoginIdentity;
use crate::domain::auth::value_objects::user::user_status::UserStatus;
use crate::infrastructure::errors::InfraError;

pub struct LoginCase {
    user_repo: Arc<dyn UserRepository>,
    auth_access_token_service: Arc<dyn AccessTokenService>,
    auth_refresh_token_service: Arc<dyn RefreshTokenService>,
    auth_password_service: Arc<dyn PasswordService>,
}

impl LoginCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        auth_access_token_service: Arc<dyn AccessTokenService>,
        auth_refresh_token_service: Arc<dyn RefreshTokenService>,
        auth_password_service: Arc<dyn PasswordService>,
    ) -> Self {
        LoginCase {
            user_repo,
            auth_access_token_service,
            auth_refresh_token_service,
            auth_password_service,
        }
    }
    pub async fn execute(&self, cmd: LoginCommand) -> CaseResult<LoginResult> {
        let existing_user: Option<User> = match cmd.login_identity {
            LoginIdentity::UserName(user_name) => self
                .user_repo
                .find_by_name(&user_name)
                .await
                .map_err(InfraError::from)?,
            LoginIdentity::UserEmail(user_email) => self
                .user_repo
                .find_by_email(&user_email)
                .await
                .map_err(InfraError::from)?,
        };
        let user = match existing_user {
            Some(user) => user,
            None => return Err(CaseError::UserNotFound),
        };
        if user.status().value().to_owned() != UserStatus::Active {
            return Err(CaseError::EmailNotVerified);
        }
        if !self
            .auth_password_service
            .compare(cmd.plain_password, user.password_hash().to_owned())
            .map_err(InfraError::from)?
        {
            return Err(CaseError::CredentialsInvalid);
        }
        let access_token = self
            .auth_access_token_service
            .encode_access_token(user.id().to_owned())
            .map_err(InfraError::from)?;
        let refresh_token = self
            .auth_refresh_token_service
            .generate_refresh_token()
            .map_err(InfraError::from)?;
        Ok(LoginResult {
            user_email: user.email().to_owned(),
            access_token,
            refresh_token,
        })
    }
}
