use std::str::FromStr;

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum UserIdError {
    #[error("get user id from &str error")]
    GetUserIdFromStrError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserId(RecordId);

impl UserId {
    pub fn new() -> Self {
        let table: &str = "user";
        let key: Uuid = Uuid::new_v4();
        let record_id: RecordId = RecordId::from_table_key(table, key);
        UserId(record_id)
    }
    pub fn from_raw_id(raw_id: &str) -> Result<Self, UserIdError> {
        let record_id: RecordId = RecordId::from_str(raw_id).map_err(|e| {
            error!("get user id from &str error: {}", e);
            UserIdError::GetUserIdFromStrError
        })?;
        Ok(UserId(record_id))
    }
    pub fn value(&self) -> &RecordId {
        &self.0
    }
}
