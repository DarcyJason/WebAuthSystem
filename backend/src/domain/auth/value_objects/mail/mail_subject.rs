#[derive(Debug, Clone)]
pub struct MailSubject(String);

impl MailSubject {
    pub fn new(mail_subject: impl Into<String>) -> Self {
        Self(mail_subject.into())
    }
    pub fn value(&self) -> String {
        self.0.to_owned()
    }
}
