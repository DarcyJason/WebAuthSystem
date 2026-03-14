use crate::domain::auth::value_objects::user::user_email::UserEmail;
use crate::domain::auth::value_objects::user::user_name::UserName;
use crate::domain::common::time::timestamp::Timestamp;

pub struct GetMeResult {
    pub user_name: UserName,
    pub user_email: UserEmail,
    pub created_at: Timestamp,
}
