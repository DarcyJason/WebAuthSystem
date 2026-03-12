use thiserror::Error;

#[derive(Debug, Error)]
pub enum MailServiceError {
    #[error("Send email failed")]
    SendEmailFailed,
    #[error("System owner email invalid")]
    SystemOwnerEmailInvalid,
}
