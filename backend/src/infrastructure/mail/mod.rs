pub mod verification_template;

use crate::domain::auth::entities::mail::mail_content::MailContent;
use crate::domain::auth::entities::mail::mail_subject::MailSubject;
use crate::domain::auth::services::mail_service::AuthMailService;
use crate::domain::user::entities::user::user_email::UserEmail;
use crate::infrastructure::errors::mail_service_error::MailServiceError;
use async_trait::async_trait;
use resend_rs::{Resend, types::CreateEmailBaseOptions};

pub struct MailService {
    mail_client: Resend,
    system_owner_email: String,
}

impl MailService {
    pub fn new(mail_client: Resend, system_owner_email: String) -> Self {
        MailService {
            mail_client,
            system_owner_email,
        }
    }
}

#[async_trait]
impl AuthMailService for MailService {
    async fn send_email(
        &self,
        to: Vec<UserEmail>,
        mail_subject: MailSubject,
        mail_content: MailContent,
    ) -> Result<(), MailServiceError> {
        let from: String = UserEmail::new(&self.system_owner_email)
            .map_err(|_| MailServiceError::SystemOwnerEmailInvalid)?
            .value()
            .to_owned();
        let to: Vec<String> = to.iter().map(|to| to.value().to_owned()).collect();
        let subject = mail_subject.value().to_owned();
        let mail_content = mail_content.value().to_owned();
        let email = CreateEmailBaseOptions::new(from, to, subject).with_html(&mail_content);
        if self.mail_client.emails.send(email).await.is_err() {
            return Err(MailServiceError::SendEmailFailed);
        }
        Ok(())
    }
}
