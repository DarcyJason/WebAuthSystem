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
