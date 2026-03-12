use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResetToken(String);

impl ResetToken {
    pub fn new() -> Self {
        ResetToken(Uuid::new_v4().to_string())
    }
    pub fn from(verification_token: impl Into<String>) -> Self {
        ResetToken(verification_token.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Default for ResetToken {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ResetToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
