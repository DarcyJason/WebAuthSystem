use crate::{
    application::{
        commands::auth::register_command::RegisterCommand,
        errors::{AppError, AppResult},
        results::commands_results::auth::register_result::RegisterResult,
    },
    domain::auth::{
        entities::user::User, repositories::db::user_repo::UserRepository,
        services::password_service::AuthPasswordService,
    },
};
use std::sync::Arc;

pub struct RegisterCase {
    user_repo: Arc<dyn UserRepository>,
    auth_password_service: Arc<dyn AuthPasswordService>,
}

impl RegisterCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        auth_password_service: Arc<dyn AuthPasswordService>,
    ) -> Self {
        RegisterCase {
            user_repo,
            auth_password_service,
        }
    }
    pub async fn execute(&self, cmd: RegisterCommand) -> AppResult<RegisterResult> {
        let existing_user = self
            .user_repo
            .find_user_by_name_or_email(&cmd.name, &cmd.email)
            .await
            .map_err(|_| AppError::SurrealDBError)?;
        if existing_user.is_some() {
            return Err(AppError::UserAlreadyExists);
        }
        let user_password_hash = self
            .auth_password_service
            .hash(cmd.plain_password)
            .map_err(|_| AppError::HashPasswordFailed)?;
        let user = User::new(cmd.name, cmd.email, user_password_hash);
        let created_result = self
            .user_repo
            .save_user(user)
            .await
            .map_err(|_| AppError::SurrealDBError)?;
        let user = match created_result {
            Some(user) => user,
            None => return Err(AppError::CreateUserFailed),
        };
        Ok(RegisterResult {
            user_email: user.email().to_owned(),
        })
    }
}
