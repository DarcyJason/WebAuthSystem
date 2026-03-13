use crate::domain::user::entities::user::user_email::UserEmail;

#[derive(Debug, Clone)]
pub struct RegisterResult {
    pub user_email: UserEmail,
}
