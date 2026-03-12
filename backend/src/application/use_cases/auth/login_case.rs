use std::sync::Arc;

use crate::domain::auth::errors::AuthDomainError;
use crate::domain::errors::DomainError;
use crate::domain::user::repositories::user_repository::UserRepository;
use crate::infrastructure::errors::InfraError;
use crate::{
    application::{
        commands::auth::login_command::LoginCommand,
        errors::{CaseError, CaseResult},
        results::commands_results::auth::login_result::LoginResult,
    },
    domain::auth::{
        services::{
            password_service::AuthPasswordService,
            token_service::{AuthAccessTokenService, AuthRefreshTokenService},
        },
        value_objects::login_identity::LoginIdentity,
    },
    domain::user::entities::user::User,
};

pub struct LoginCase {
    user_repo: Arc<dyn UserRepository>,
    auth_access_token_service: Arc<dyn AuthAccessTokenService>,
    auth_refresh_token_service: Arc<dyn AuthRefreshTokenService>,
    auth_password_service: Arc<dyn AuthPasswordService>,
}

impl LoginCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        auth_access_token_service: Arc<dyn AuthAccessTokenService>,
        auth_refresh_token_service: Arc<dyn AuthRefreshTokenService>,
        auth_password_service: Arc<dyn AuthPasswordService>,
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
        if !user.status().value().to_owned() {
            return Err(CaseError::EmailNotVerified);
        }
        if !self
            .auth_password_service
            .compare(cmd.plain_password, user.password_hash().to_owned())
            .map_err(AuthDomainError::from)
            .map_err(DomainError::from)?
        {
            return Err(CaseError::CredentialsInvalid);
        }
        let access_token = self
            .auth_access_token_service
            .encode_access_token(user.id().to_owned())
            .map_err(AuthDomainError::from)
            .map_err(DomainError::from)?;
        let refresh_token = self
            .auth_refresh_token_service
            .generate_refresh_token()
            .map_err(AuthDomainError::from)
            .map_err(DomainError::from)?;
        Ok(LoginResult {
            user_email: user.email().to_owned(),
            access_token,
            refresh_token,
        })
    }
}
