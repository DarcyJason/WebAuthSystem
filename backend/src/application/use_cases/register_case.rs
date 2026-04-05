use crate::application::commands::register_command::RegisterCommand;
use crate::application::error::ApplicationResult;
use crate::application::results::register_result::RegisterResult;
use crate::infrastructure::layered::user_repository::LayeredUserRepository;
use crate::infrastructure::security::password::Argon2PasswordService;

pub struct RegisterCase {
    user_repo: LayeredUserRepository,
    password_service: Argon2PasswordService,
}

impl RegisterCase {
    pub fn new(user_repo: LayeredUserRepository, password_service: Argon2PasswordService) -> Self {
        RegisterCase {
            user_repo,
            password_service,
        }
    }
    pub async fn execute(&self, cmd: RegisterCommand) -> ApplicationResult<RegisterResult> {
        Ok(RegisterResult {})
    }
}
