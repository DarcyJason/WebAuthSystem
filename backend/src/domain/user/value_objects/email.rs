use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub enum EmailError {
    EmailIsInvalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email(String);
impl Email {
    pub fn new(email: String) -> Result<Self, EmailError> {
        let email = email.trim().to_string();
        if !email.contains("@") {
            return Err(EmailError::EmailIsInvalid);
        }
        Ok(Email(email))
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
