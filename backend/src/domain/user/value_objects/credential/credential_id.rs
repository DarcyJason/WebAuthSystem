use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct CredentialId(Uuid);

impl CredentialId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl Default for CredentialId {
    fn default() -> Self {
        Self::new()
    }
}
