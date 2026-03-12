use async_trait::async_trait;

use crate::domain::{
    auth::value_objects::{mail_content::MailContent, mail_subject::MailSubject},
    user::value_objects::user_email::UserEmail,
};
use crate::infrastructure::errors::mail_service_error::MailServiceError;

#[async_trait]
pub trait AuthMailService: Send + Sync {
    async fn send_email(
        &self,
        to: Vec<UserEmail>,
        mail_subject: MailSubject,
        mail_content: MailContent,
    ) -> Result<(), MailServiceError>;
}
