use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleName(String);

impl RoleName {
    pub fn new(role_name: impl Into<String>) -> Self {
        RoleName(role_name.into())
    }
}
