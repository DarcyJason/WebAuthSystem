use std::sync::Arc;

use crate::domain::auth::services::mail_service::AuthMailService;

pub struct SendEmailCase {
    auth_mail_service: Arc<dyn AuthMailService>,
}

impl SendEmailCase {
    pub fn new(auth_mail_service: Arc<dyn AuthMailService>) -> Self {
        SendEmailCase { auth_mail_service }
    }
}
