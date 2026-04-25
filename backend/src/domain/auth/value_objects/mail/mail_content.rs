#[derive(Debug, Clone)]
pub struct MailContent(String);

impl MailContent {
    pub fn new(mail_content: impl Into<String>) -> Self {
        Self(mail_content.into())
    }
    pub fn value(&self) -> String {
        self.0.to_owned()
    }
}
