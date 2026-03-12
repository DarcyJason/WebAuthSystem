use crate::domain::auth::errors::AuthDomainError;
use crate::domain::errors::DomainError;
use crate::domain::user::repositories::user_repository::UserRepository;
use crate::infrastructure::errors::InfraError;
use crate::{
    application::{
        commands::auth::register_command::RegisterCommand,
        errors::{CaseError, CaseResult},
        results::commands_results::auth::register_result::RegisterResult,
    },
    domain::auth::services::password_service::AuthPasswordService,
    domain::user::entities::user::User,
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
    pub async fn execute(&self, cmd: RegisterCommand) -> CaseResult<RegisterResult> {
        let existing_user = self
            .user_repo
            .find_by_name_or_email(&cmd.name, &cmd.email)
            .await
            .map_err(InfraError::from)?;
        if existing_user.is_some() {
            return Err(CaseError::UserAlreadyExists);
        }
        let user_password_hash = self
            .auth_password_service
            .hash(cmd.plain_password)
            .map_err(AuthDomainError::from)
            .map_err(DomainError::from)?;
        let user = User::new(cmd.name, cmd.email, user_password_hash);
        let created_result = self.user_repo.save(user).await.map_err(InfraError::from)?;
        let user = match created_result {
            Some(user) => user,
            None => return Err(CaseError::UserCreatedFailed),
        };
        Ok(RegisterResult {
            user_email: user.email().to_owned(),
        })
    }
}
