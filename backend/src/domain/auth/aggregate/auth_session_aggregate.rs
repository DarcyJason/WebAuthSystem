use crate::domain::auth::event::AuthEvent;
use crate::domain::auth::value_objects::device::device_id::DeviceId;
use crate::domain::auth::value_objects::session::session_id::SessionId;
use crate::domain::auth::value_objects::session::session_status::SessionStatus;
use crate::domain::auth::value_objects::tokens::refresh_token::RefreshToken;
use crate::domain::auth::value_objects::user::user_id::UserId;
use crate::domain::common::time::timestamp::Timestamp;

pub struct AuthSession {
    pub session_id: SessionId,
    pub user_id: UserId,
    pub device_id: DeviceId,
    pub refresh_token: RefreshToken,
    pub status: SessionStatus,
    pub created_at: Timestamp,
    pub expires_at: Timestamp,
    pub domain_events: Vec<AuthEvent>,
}

impl AuthSession {
    pub fn pull_events(&mut self) -> Vec<AuthEvent> {
        // 事件只消费一次
        std::mem::take(&mut self.domain_events)
    }
}
