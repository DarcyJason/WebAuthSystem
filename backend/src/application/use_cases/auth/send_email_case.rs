use std::sync::Arc;

use crate::{
    application::commands::auth::send_email_command::SendEmailCommand,
    domain::auth::{
        repositories::cache::email_verify_cache::EmailVerifyCache,
        services::mail_service::AuthMailService,
    },
};

pub struct SendEmailCase {
    auth_mail_service: Arc<dyn AuthMailService>,
    auth_email_verify_cache: Arc<dyn EmailVerifyCache>,
}

impl SendEmailCase {
    pub fn new(
        auth_mail_service: Arc<dyn AuthMailService>,
        auth_email_verify_cache: Arc<dyn EmailVerifyCache>,
    ) -> Self {
        SendEmailCase {
            auth_mail_service,
            auth_email_verify_cache,
        }
    }
    pub async fn execute(&self, send_email_command: SendEmailCommand) {}
}
