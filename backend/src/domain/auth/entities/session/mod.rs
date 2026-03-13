use crate::domain::auth::entities::session::session_id::SessionId;
use crate::domain::auth::entities::session::session_name::SessionName;

pub mod session_id;
pub mod session_name;

pub struct Session {
    pub id: SessionId,
    pub name: SessionName,
}
