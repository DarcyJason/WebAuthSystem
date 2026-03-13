use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailContent(String);

impl MailContent {
    pub fn new(mail_content: impl Into<String>) -> Self {
        MailContent(mail_content.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
