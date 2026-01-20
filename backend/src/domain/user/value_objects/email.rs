use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub enum EmailError {
    EmailRequired,
    EmailInvalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email(String);
impl Email {
    pub fn new(email: String) -> Result<Self, EmailError> {
        if email.is_empty() {
            return Err(EmailError::EmailRequired);
        }
        if !email.contains("@") {
            return Err(EmailError::EmailInvalid);
        }
        Ok(Email(email))
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
