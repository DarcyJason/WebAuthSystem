use crate::domain::user::value_objects::user_email::UserEmail;

#[derive(Debug, Clone)]
pub struct RegisterResult {
    pub user_email: UserEmail,
}
