use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub enum UsernameError {
    UsernameIsInvalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Username(String);
impl Username {
    pub fn new(username: String) -> Result<Self, UsernameError> {
        let username = username.trim().to_string();
        if username.contains("@") {
            return Err(UsernameError::UsernameIsInvalid);
        }
        Ok(Username(username))
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
