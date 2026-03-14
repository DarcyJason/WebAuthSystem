use serde::{Deserialize, Serialize};
use std::str::FromStr;
use surrealdb::RecordId;
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum PermissionIdError {
    #[error("get role id from &str error")]
    GetRoleIdFromStrError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionId(RecordId);

impl PermissionId {
    pub fn new() -> Self {
        let table: &str = "role";
        let key: Uuid = Uuid::new_v4();
        let record_id: RecordId = RecordId::from_table_key(table, key);
        PermissionId(record_id)
    }
    pub fn from_raw_id(raw_id: &str) -> Result<Self, PermissionIdError> {
        let record_id: RecordId = RecordId::from_str(raw_id).map_err(|e| {
            error!("get role id from &str error: {}", e);
            PermissionIdError::GetRoleIdFromStrError
        })?;
        Ok(PermissionId(record_id))
    }
    pub fn value(&self) -> &RecordId {
        &self.0
    }
}

impl Default for PermissionId {
    fn default() -> Self {
        Self::new()
    }
}
