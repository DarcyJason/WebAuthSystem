#[derive(Debug, Clone)]
pub struct MailSubject(String);

impl MailSubject {
    pub fn new(mail_subject: impl Into<String>) -> Self {
        MailSubject(mail_subject.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
