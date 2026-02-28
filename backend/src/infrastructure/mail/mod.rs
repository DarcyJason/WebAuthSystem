pub mod verification_template;

use async_trait::async_trait;
use resend_rs::{Resend, types::CreateEmailBaseOptions};

use crate::domain::{
    auth::{
        services::mail_service::{AuthMailService, AuthMailServiceError},
        value_objects::{mail_content::MailContent, mail_subject::MailSubject},
    },
    user::value_objects::user_email::UserEmail,
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
    async fn send_email(
        &self,
        from: UserEmail,
        to: Vec<UserEmail>,
        mail_subject: MailSubject,
        mail_content: MailContent,
    ) -> Result<(), AuthMailServiceError> {
        let from = from.value().to_owned();
        let to: Vec<String> = to.iter().map(|to| to.value().to_owned()).collect();
        let subject = mail_subject.value().to_owned();
        let mail_content = mail_content.value().to_owned();
        let email = CreateEmailBaseOptions::new(from, to, subject).with_html(&mail_content);
        if self.mail_client.emails.send(email).await.is_err() {
            return Err(AuthMailServiceError::SendEmailFailed);
        }
        Ok(())
    }
}
