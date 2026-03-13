use crate::domain::auth::entities::mail::mail_content::MailContent;
use crate::domain::auth::entities::mail::mail_id::MailId;
use crate::domain::auth::entities::mail::mail_subject::MailSubject;
use crate::domain::common::time::timestamp::Timestamp;
use serde::{Deserialize, Serialize};

pub mod mail_content;
pub mod mail_id;
pub mod mail_subject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mail {
    pub id: MailId,
    pub subject: MailSubject,
    pub content: MailContent,
    pub created_at: Timestamp,
}
