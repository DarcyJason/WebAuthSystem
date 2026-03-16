use crate::domain::rbac::errors::RoleCodeError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleCode(String);

impl RoleCode {
    pub fn new(code: impl Into<String>) -> Result<Self, RoleCodeError> {
        let code = code.into();
        if code.is_empty() || !code.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
            return Err(RoleCodeError::InvalidFormat);
        }
        Ok(Self(code))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
