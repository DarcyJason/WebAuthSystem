use std::sync::Arc;

use crate::domain::auth::repository::user_repository::UserRepository;

pub struct ResetPasswordCase {
    user_repo: Arc<dyn UserRepository>,
}

impl ResetPasswordCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        ResetPasswordCase { user_repo }
    }
    pub async fn execute(&self) {}
}
