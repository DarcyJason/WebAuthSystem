use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailSubject(String);

impl MailSubject {
    pub fn new(mail_subject: impl Into<String>) -> Self {
        MailSubject(mail_subject.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
