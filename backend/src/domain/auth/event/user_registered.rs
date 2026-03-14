use crate::domain::auth::value_objects::user::user_id::UserId;

#[derive(Debug)]
pub struct UserRegistered {
    pub user_id: UserId,
}
