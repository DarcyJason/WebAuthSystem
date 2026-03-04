use std::str::FromStr;

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum UserIdError {
    #[error("Get user id from &str failed")]
    GetUserIdFromStrFailed,
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
        let record_id: RecordId =
            RecordId::from_str(raw_id).map_err(|_| UserIdError::GetUserIdFromStrFailed)?;
        Ok(UserId(record_id))
    }
    pub fn value(&self) -> &RecordId {
        &self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}
