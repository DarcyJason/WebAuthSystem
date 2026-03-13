use crate::domain::user::entities::user::user_email::UserEmail;

pub struct SendVerificationEmailCommand {
    pub email: UserEmail,
}
