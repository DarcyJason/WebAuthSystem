use crate::domain::auth::value_objects::mail::Mail;
use crate::domain::error::DomainResult;
use crate::domain::identities::value_objects::user::user_email::UserEmail;
use async_trait::async_trait;

#[async_trait]
pub trait MailService: Send + Sync {
    async fn send_email(&self, to: Vec<UserEmail>, mail: Mail) -> DomainResult<()>;
}
