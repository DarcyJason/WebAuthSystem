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
}
