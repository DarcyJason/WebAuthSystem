pub mod verification_email_template;

use crate::domain::auth::services::mail_service::MailService;
use crate::domain::auth::value_objects::mail::Mail;
use crate::domain::error::{DomainResult, SendEmailFailedSnafu, SystemOwnerEmailInvalidSnafu};
use crate::domain::user::value_objects::user::user_email::UserEmail;
use async_trait::async_trait;
use resend_rs::Resend;
use resend_rs::types::CreateEmailBaseOptions;

#[derive(Debug, Clone)]
pub struct ResendMailService {
    mail_client: Resend,
    system_owner_email: String,
}

impl ResendMailService {
    pub fn new(mail_client: Resend, system_owner_email: String) -> Self {
        ResendMailService {
            mail_client,
            system_owner_email,
        }
    }
}

#[async_trait]
impl MailService for ResendMailService {
    async fn send_email(&self, to: Vec<UserEmail>, mail: Mail) -> DomainResult<()> {
        let from = UserEmail::new(&self.system_owner_email)
            .map_err(|e| {
                SystemOwnerEmailInvalidSnafu {
                    message: e.to_string(),
                }
                .build()
            })?
            .value()
            .to_owned();
        let to: Vec<String> = to.iter().map(|to| to.value().to_owned()).collect();
        let subject = mail.subject().value().to_owned();
        let content = mail.content().value().to_owned();
        let email = CreateEmailBaseOptions::new(from, to, subject).with_html(&content);
        if let Err(e) = self.mail_client.emails.send(email).await {
            return Err(SendEmailFailedSnafu {
                message: e.to_string(),
            }
            .build());
        }
        Ok(())
    }
}
