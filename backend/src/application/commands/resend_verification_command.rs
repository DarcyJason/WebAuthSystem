use crate::domain::user::value_objects::user::user_email::UserEmail;

pub struct ResendVerificationCommand {
    pub email: UserEmail,
}
