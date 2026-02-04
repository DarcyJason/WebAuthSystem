use serde::{Deserialize, Serialize};

use crate::domain::rbac::value_objects::permission_code::PermissionCode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub code: PermissionCode,
    pub description: String,
}

impl Permission {
    pub fn new(code: PermissionCode, description: impl Into<String>) -> Self {
        Self {
            code,
            description: description.into(),
        }
    }
    pub fn code(&self) -> &PermissionCode {
        &self.code
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}
