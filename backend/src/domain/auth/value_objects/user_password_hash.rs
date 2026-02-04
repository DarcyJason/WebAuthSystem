use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPasswordHash(String);

impl UserPasswordHash {
    pub fn new(user_password_hash: impl Into<String>) -> Self {
        UserPasswordHash(user_password_hash.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
