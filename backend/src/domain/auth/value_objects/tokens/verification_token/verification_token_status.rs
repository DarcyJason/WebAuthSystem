use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "verification_token_status", rename_all = "PascalCase")]
pub enum VerificationTokenStatus {
    Unused,
    Used,
    Invalid,
}

impl VerificationTokenStatus {
    pub fn new() -> Self {
        VerificationTokenStatus::Unused
    }
    pub fn use_it(&self) -> Self {
        VerificationTokenStatus::Used
    }
    pub fn value(&self) -> &VerificationTokenStatus {
        self
    }
}

impl Default for VerificationTokenStatus {
    fn default() -> Self {
        VerificationTokenStatus::new()
    }
}
