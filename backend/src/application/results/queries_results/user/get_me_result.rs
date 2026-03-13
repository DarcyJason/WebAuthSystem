use crate::domain::common::time::timestamp::Timestamp;
use crate::domain::user::entities::user::user_email::UserEmail;
use crate::domain::user::entities::user::user_name::UserName;

pub struct GetMeResult {
    pub user_name: UserName,
    pub user_email: UserEmail,
    pub created_at: Timestamp,
}
