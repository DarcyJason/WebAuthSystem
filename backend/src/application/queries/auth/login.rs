use serde::Serialize;

use crate::domain::user::entities::User;

#[derive(Debug, Serialize)]
pub struct LoginResult {
    pub user_id: String,
    pub username: String,
    pub email: String,
}

impl From<User> for LoginResult {
    fn from(user: User) -> Self {
        LoginResult {
            user_id: user.id().to_string(),
            username: user.username().to_string(),
            email: user.email().to_string(),
        }
    }
}
