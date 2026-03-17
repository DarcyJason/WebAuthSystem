use async_trait::async_trait;

use crate::domain::auth::value_objects::mail::Mail;
use crate::domain::auth::value_objects::user::user_email::UserEmail;
use crate::infrastructure::errors::mail_service_error::MailServiceError;

#[async_trait]
pub trait MailService: Send + Sync {
    async fn send_email(&self, to: Vec<UserEmail>, mail: Mail) -> Result<(), MailServiceError>;
}
