use crate::domain::auth::value_objects::mail::mail_content::MailContent;
use crate::domain::auth::value_objects::mail::mail_subject::MailSubject;

pub mod mail_content;
pub mod mail_subject;

#[derive(Debug, Clone)]
pub struct Mail {
    subject: MailSubject,
    content: MailContent,
}

impl Mail {
    pub fn new(subject: MailSubject, content: MailContent) -> Self {
        Mail { subject, content }
    }
    pub fn subject(&self) -> MailSubject {
        self.subject.to_owned()
    }
    pub fn content(&self) -> MailContent {
        self.content.to_owned()
    }
}
