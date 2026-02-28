use crate::domain::user::value_objects::user_email::UserEmail;

pub struct SendVerificationEmailCommand {
    pub email: UserEmail,
}
