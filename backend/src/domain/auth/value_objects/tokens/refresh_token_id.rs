use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenId(Uuid);

impl RefreshTokenId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from(id: Uuid) -> Self {
        Self(id)
    }

    pub fn value(&self) -> &Uuid {
        self.0.as_ref()
    }
}
