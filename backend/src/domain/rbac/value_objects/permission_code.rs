use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PermissionCodeError {
    #[error("invalid permission code format")]
    InvalidFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PermissionCode(String);

impl PermissionCode {
    pub fn new(code: impl Into<String>) -> Result<Self, PermissionCodeError> {
        let code = code.into();
        if !code.contains(':') {
            return Err(PermissionCodeError::InvalidFormat);
        }
        Ok(Self(code))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
