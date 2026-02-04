use async_trait::async_trait;

use crate::domain::auth::value_objects::{
    mail_content::MailContent, mail_id::MailId, mail_subject::MailSubject, user_email::UserEmail,
};

pub enum AuthMailServiceError {
    SendEmailFailed,
}

#[async_trait]
pub trait AuthMailService: Send + Sync {
    async fn send(
        &self,
        from: UserEmail,
        to: Vec<UserEmail>,
        mail_subject: MailSubject,
        mail_content: MailContent,
    ) -> Result<MailId, AuthMailServiceError>;
}
