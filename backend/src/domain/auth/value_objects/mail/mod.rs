pub mod mail_content;
pub mod mail_subject;

use crate::domain::auth::value_objects::mail::mail_content::MailContent;
use crate::domain::auth::value_objects::mail::mail_subject::MailSubject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mail {
    pub subject: MailSubject,
    pub content: MailContent,
}

impl Mail {
    pub fn new(subject: MailSubject, content: MailContent) -> Self {
        Mail { subject, content }
    }
}
