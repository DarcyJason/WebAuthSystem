use crate::domain::{
    auth::value_objects::{user_email::UserEmail, user_name::UserName},
    common::time::timestamp::Timestamp,
};

pub struct GetMeResult {
    pub user_name: UserName,
    pub user_email: UserEmail,
    pub created_at: Timestamp,
}
