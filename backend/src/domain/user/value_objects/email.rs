use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum EmailError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email(String);
impl Email {
    pub fn new(email: String) -> Result<Self, EmailError> {
        if email.is_empty() {
            error!("[domain] email error: {}", EmailError::EmailRequired);
            return Err(EmailError::EmailRequired);
        }
        if !email.contains("@") {
            error!("[domain] email error: {}", EmailError::EmailInvalid);
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
