use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    Active,
    NotVerified,
    Locked,
}

impl UserStatus {
    pub fn new() -> Self {
        UserStatus::Locked
    }
    pub fn value(&self) -> &Self {
        &self
    }
}
