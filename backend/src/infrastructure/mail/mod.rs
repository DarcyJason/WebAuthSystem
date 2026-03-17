pub mod verification_template;

use crate::domain::auth::value_objects::user::user_email::UserEmail;
use crate::domain::auth::{service::mail_service::MailService, value_objects::mail::Mail};
use crate::infrastructure::errors::mail_service_error::MailServiceError;
use async_trait::async_trait;
use resend_rs::{Resend, types::CreateEmailBaseOptions};

pub struct DefaultMailService {
    mail_client: Resend,
    system_owner_email: String,
}

impl DefaultMailService {
    pub fn new(mail_client: Resend, system_owner_email: String) -> Self {
        DefaultMailService {
            mail_client,
            system_owner_email,
        }
    }
}

#[async_trait]
impl MailService for DefaultMailService {
    async fn send_email(&self, to: Vec<UserEmail>, mail: Mail) -> Result<(), MailServiceError> {
        let from: String = UserEmail::new(&self.system_owner_email)
            .map_err(|_| MailServiceError::SystemOwnerEmailInvalid)?
            .value()
            .to_owned();
        let to: Vec<String> = to.iter().map(|to| to.value().to_owned()).collect();
        let subject = mail.subject.value().to_owned();
        let mail_content = mail.content.value().to_owned();
        let email = CreateEmailBaseOptions::new(from, to, subject).with_html(&mail_content);
        if self.mail_client.emails.send(email).await.is_err() {
            return Err(MailServiceError::SendEmailFailed);
        }
        Ok(())
    }
}
