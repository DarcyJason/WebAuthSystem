use std::str::FromStr;

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RoleIdError {
    #[error("get role id from &str error")]
    GetRoleIdFromStrError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleId(RecordId);

impl RoleId {
    pub fn new() -> Self {
        let table: &str = "role";
        let key: Uuid = Uuid::new_v4();
        let record_id: RecordId = RecordId::from_table_key(table, key);
        RoleId(record_id)
    }
    pub fn from_raw_id(raw_id: &str) -> Result<Self, RoleIdError> {
        let record_id: RecordId = RecordId::from_str(raw_id).map_err(|e| {
            error!("get role id from &str error: {}", e);
            RoleIdError::GetRoleIdFromStrError
        })?;
        Ok(RoleId(record_id))
    }
    pub fn value(&self) -> &RecordId {
        &self.0
    }
}
