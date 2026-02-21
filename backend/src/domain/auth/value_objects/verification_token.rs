use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationToken(String);

impl VerificationToken {
    pub fn new() -> Self {
        VerificationToken(Uuid::new_v4().to_string())
    }
    pub fn from(verification_token: impl Into<String>) -> Self {
        VerificationToken(verification_token.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Display for VerificationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
