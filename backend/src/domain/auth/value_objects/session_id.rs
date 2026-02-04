use std::str::FromStr;

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum SessionIdError {
    #[error("get session id from &str error")]
    GetSessionIdFromStrError,
}

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
            error!("get session id from &str error: {}", e);
            SessionIdError::GetSessionIdFromStrError
        })?;
        Ok(SessionId(record_id))
    }
    pub fn value(&self) -> &RecordId {
        &self.0
    }
}
