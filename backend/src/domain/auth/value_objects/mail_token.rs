use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MailToken(String);

impl MailToken {
    pub fn new() -> Self {
        MailToken(Uuid::new_v4().to_string())
    }
    pub fn from(mail_token: impl Into<String>) -> Self {
        MailToken(mail_token.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
