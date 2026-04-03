use crate::application::auth::commands::register_command::RegisterCommand;
use crate::application::auth::results::register_result::RegisterResult;
use crate::application::error::ApplicationResult;
use crate::domain::identities::repositories::user_repository::UserRepository;
use std::sync::Arc;

pub struct RegisterCase {
    user_repo: Arc<dyn UserRepository>,
}

impl RegisterCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        RegisterCase { user_repo }
    }
    pub async fn execute(&self, cmd: RegisterCommand) -> ApplicationResult<RegisterResult> {
        Ok(RegisterResult {})
    }
}
