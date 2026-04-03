use crate::application::auth::commands::login_command::LoginCommand;
use crate::application::auth::results::login_result::LoginResult;
use crate::application::error::ApplicationResult;
use crate::domain::identities::repositories::user_repository::UserRepository;
use std::sync::Arc;

pub struct LoginCase {
    user_repo: Arc<dyn UserRepository>,
}

impl LoginCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        LoginCase { user_repo }
    }
    pub async fn execute(&self, cmd: LoginCommand) -> ApplicationResult<LoginResult> {
        Ok(LoginResult {})
    }
}
