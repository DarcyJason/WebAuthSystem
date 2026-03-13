#[derive(Debug, Clone)]
pub struct MailId(String);

impl MailId {
    pub fn new(mail_id: impl Into<String>) -> Self {
        MailId(mail_id.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
