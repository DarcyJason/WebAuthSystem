use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationTokenUsed(bool);

impl VerificationTokenUsed {
    pub fn new() -> Self {
        Self(false)
    }
    pub fn value(&self) -> bool {
        self.0
    }
}

impl Default for VerificationTokenUsed {
    fn default() -> Self {
        VerificationTokenUsed::new()
    }
}
