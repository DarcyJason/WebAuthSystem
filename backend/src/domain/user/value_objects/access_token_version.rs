use serde::{Deserialize, Serialize};
use uuid::{Timestamp, Uuid};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
pub struct AccessTokenVersion(Uuid);

impl AccessTokenVersion {
    pub fn new() -> Self {
        Self(Uuid::new_v7(Timestamp::now(uuid::NoContext)))
    }
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl Default for AccessTokenVersion {
    fn default() -> Self {
        Self::new()
    }
}
