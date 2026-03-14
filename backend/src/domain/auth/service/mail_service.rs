use async_trait::async_trait;

use crate::domain::auth::value_objects::mail::mail_content::MailContent;
use crate::domain::auth::value_objects::mail::mail_subject::MailSubject;
use crate::domain::auth::value_objects::user::user_email::UserEmail;
use crate::infrastructure::errors::mail_service_error::MailServiceError;

#[async_trait]
pub trait MailService: Send + Sync {
    async fn send_email(
        &self,
        to: Vec<UserEmail>,
        mail_subject: MailSubject,
        mail_content: MailContent,
    ) -> Result<(), MailServiceError>;
}
