use crate::domain::user::entities::User;
use crate::domain::user::value_objects::email::Email;
use crate::domain::user::value_objects::username::Username;
use surrealdb::RecordId;

#[derive(Debug, Clone)]
pub struct RegisterResult {
    pub user_id: RecordId,
    pub username: Username,
    pub email: Email,
}

impl From<User> for RegisterResult {
    fn from(user: User) -> Self {
        RegisterResult {
            user_id: user.id().to_owned(),
            username: user.username().to_owned(),
            email: user.email().to_owned(),
        }
    }
}
