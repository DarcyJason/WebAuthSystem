#[derive(Debug, Clone)]
pub struct MailContent(String);

impl MailContent {
    pub fn new(mail_content: impl Into<String>) -> Self {
        MailContent(mail_content.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
