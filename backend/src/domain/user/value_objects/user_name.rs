use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub enum UserNameError {
    UserNameInvalid,
    UserNameTooLong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserName(String);

impl UserName {
    pub fn new(user_name: impl Into<String>) -> Result<Self, UserNameError> {
        let user_name: String = user_name.into();
        if user_name.contains("@") {
            return Err(UserNameError::UserNameInvalid);
        }
        if user_name.len() > 20 {
            return Err(UserNameError::UserNameTooLong);
        }
        Ok(UserName(user_name))
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
