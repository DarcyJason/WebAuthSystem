use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatus(bool);

impl UserStatus {
    pub fn new(status: bool) -> Self {
        UserStatus(status)
    }
    pub fn value(&self) -> &bool {
        &self.0
    }
}
