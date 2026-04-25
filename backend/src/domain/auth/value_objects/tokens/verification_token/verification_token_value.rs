use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationTokenValue(String);

impl VerificationTokenValue {
    pub fn new() -> Self {
        VerificationTokenValue(Uuid::new_v4().to_string())
    }
    pub fn from(verification_token_value: impl Into<String>) -> Self {
        VerificationTokenValue(verification_token_value.into())
    }
    pub fn value(&self) -> String {
        self.0.to_owned()
    }
}

impl Default for VerificationTokenValue {
    fn default() -> Self {
        VerificationTokenValue::new()
    }
}

impl Display for VerificationTokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
