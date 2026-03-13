use crate::domain::user::entities::user::user_email::UserEmail;

pub struct ForgotPasswordCommand {
    pub email: UserEmail,
}
