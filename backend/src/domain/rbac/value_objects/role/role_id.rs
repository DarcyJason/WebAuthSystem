use crate::domain::rbac::errors::RoleIdError;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use tracing::error;
use uuid::Uuid;

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
            error!("invalid role id format: {}", e);
            RoleIdError::InvalidFormat
        })?;
        Ok(RoleId(record_id))
    }
    pub fn value(&self) -> &RecordId {
        &self.0
    }
}

impl Default for RoleId {
    fn default() -> Self {
        Self::new()
    }
}
