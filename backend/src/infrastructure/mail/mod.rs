use async_trait::async_trait;
use resend_rs::{Resend, types::CreateEmailBaseOptions};

use crate::domain::auth::{
    services::mail_service::{AuthMailService, AuthMailServiceError},
    value_objects::{
        mail_content::MailContent, mail_id::MailId, mail_subject::MailSubject,
        user_email::UserEmail,
    },
};

pub struct MailService {
    mail_client: Resend,
}

impl MailService {
    pub fn new(mail_client: Resend) -> Self {
        MailService { mail_client }
    }
}

#[async_trait]
impl AuthMailService for MailService {
    async fn send_verification(
        &self,
        from: UserEmail,
        to: Vec<UserEmail>,
        mail_subject: MailSubject,
        mail_content: MailContent,
    ) -> Result<MailId, AuthMailServiceError> {
        let from = from.value().to_owned();
        let to: Vec<String> = to.iter().map(|to| to.value().to_owned()).collect();
        let subject = mail_subject.value().to_owned();
        let mail_content = mail_content.value().to_owned();
        let email = CreateEmailBaseOptions::new(from, to, subject).with_html(&mail_content);
        let id = match self.mail_client.emails.send(email).await {
            Ok(id) => id.id.to_string(),
            Err(_) => return Err(AuthMailServiceError::SendEmailFailed),
        };
        Ok(MailId::new(id))
    }
    async fn verify(&self) {}
}
