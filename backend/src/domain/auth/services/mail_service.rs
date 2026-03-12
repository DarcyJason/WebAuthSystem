use async_trait::async_trait;
use thiserror::Error;

use crate::domain::{
    auth::value_objects::{mail_content::MailContent, mail_subject::MailSubject},
    user::value_objects::user_email::UserEmail,
};

#[derive(Debug, Error)]
pub enum MailServiceError {
    #[error("Send email failed")]
    SendEmailFailed,
    #[error("System owner email invalid")]
    SystemOwnerEmailInvalid,
}

#[async_trait]
pub trait AuthMailService: Send + Sync {
    async fn send_email(
        &self,
        to: Vec<UserEmail>,
        mail_subject: MailSubject,
        mail_content: MailContent,
    ) -> Result<(), MailServiceError>;
}
