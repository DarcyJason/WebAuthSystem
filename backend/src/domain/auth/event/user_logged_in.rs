use crate::domain::auth::value_objects::device::device_id::DeviceId;
use crate::domain::auth::value_objects::session::session_id::SessionId;
use crate::domain::auth::value_objects::user::user_id::UserId;
use crate::domain::common::time::timestamp::Timestamp;

#[derive(Debug)]
pub struct UserLoggedIn {
    pub user_id: UserId,
    pub session_id: SessionId,
    pub device_id: DeviceId,
    pub occurred_at: Timestamp,
}
