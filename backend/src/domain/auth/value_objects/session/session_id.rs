use crate::domain::auth::errors::session::session_id_error::SessionIdError;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionId(RecordId);

impl SessionId {
    pub fn new() -> Self {
        let table: &str = "session";
        let key: Uuid = Uuid::new_v4();
        let record_id: RecordId = RecordId::from_table_key(table, key);
        SessionId(record_id)
    }
    pub fn from_raw_id(raw_id: &str) -> Result<Self, SessionIdError> {
        let record_id: RecordId = RecordId::from_str(raw_id).map_err(|e| {
            error!("invalid session id format: {}", e);
            SessionIdError::InvalidFormat
        })?;
        Ok(SessionId(record_id))
    }
    pub fn value(&self) -> &RecordId {
        &self.0
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}
