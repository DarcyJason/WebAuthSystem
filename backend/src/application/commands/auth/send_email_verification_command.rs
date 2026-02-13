use crate::domain::auth::value_objects::user_email::UserEmail;

pub struct SendEmailVerificationCommand {
    pub email: UserEmail,
}
