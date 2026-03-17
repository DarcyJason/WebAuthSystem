use std::sync::Arc;

use crate::domain::auth::repository::user_repository::UserRepository;

pub struct ForgotPasswordCase {
    user_repo: Arc<dyn UserRepository>,
}

impl ForgotPasswordCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        ForgotPasswordCase { user_repo }
    }
    pub async fn execute(&self) {}
}
