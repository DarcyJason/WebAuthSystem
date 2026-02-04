use std::str::FromStr;

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DeviceIdError {
    #[error("get device id from &str error")]
    GetDeviceIdFromStrError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceId(RecordId);

impl DeviceId {
    pub fn new() -> Self {
        let table: &str = "device";
        let key: Uuid = Uuid::new_v4();
        let record_id: RecordId = RecordId::from_table_key(table, key);
        DeviceId(record_id)
    }
    pub fn from_raw_id(raw_id: &str) -> Result<Self, DeviceIdError> {
        let record_id: RecordId = RecordId::from_str(raw_id).map_err(|e| {
            error!("get device id from &str error: {}", e);
            DeviceIdError::GetDeviceIdFromStrError
        })?;
        Ok(DeviceId(record_id))
    }
    pub fn value(&self) -> &RecordId {
        &self.0
    }
}
