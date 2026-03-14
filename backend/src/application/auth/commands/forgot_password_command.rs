use crate::domain::auth::value_objects::user::user_email::UserEmail;

pub struct ForgotPasswordCommand {
    pub email: UserEmail,
}
