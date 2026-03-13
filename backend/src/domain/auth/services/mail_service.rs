use async_trait::async_trait;

use crate::{
    domain::{
        auth::entities::mail::{mail_content::MailContent, mail_subject::MailSubject},
        user::entities::user::user_email::UserEmail,
    },
    infrastructure::errors::mail_service_error::MailServiceError,
};

#[async_trait]
pub trait MailService: Send + Sync {
    async fn send_email(
        &self,
        to: Vec<UserEmail>,
        mail_subject: MailSubject,
        mail_content: MailContent,
    ) -> Result<(), MailServiceError>;
}
