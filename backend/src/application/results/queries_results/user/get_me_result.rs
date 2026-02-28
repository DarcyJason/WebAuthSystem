use crate::domain::{
    common::time::timestamp::Timestamp,
    user::value_objects::{user_email::UserEmail, user_name::UserName},
};

pub struct GetMeResult {
    pub user_name: UserName,
    pub user_email: UserEmail,
    pub created_at: Timestamp,
}
