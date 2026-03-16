use crate::domain::auth::value_objects::user::{user_email::UserEmail, user_id::UserId};

#[derive(Debug)]
pub struct UserRegistered {
    pub user_id: UserId,
    pub user_email: UserEmail,
}
