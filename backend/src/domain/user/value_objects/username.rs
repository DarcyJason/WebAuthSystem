use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub enum UsernameError {
    UsernameInvalid,
    UsernameTooLong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Username(String);
impl Username {
    pub fn new(username: String) -> Result<Self, UsernameError> {
        if username.contains("@") {
            return Err(UsernameError::UsernameInvalid);
        }
        if username.len() > 20 {
            return Err(UsernameError::UsernameTooLong);
        }
        Ok(Username(username))
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
