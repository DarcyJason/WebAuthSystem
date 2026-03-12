use crate::domain::auth::value_objects::plain_password::PlainPassword;
use crate::domain::auth::value_objects::reset_token::ResetToken;
use crate::domain::user::value_objects::user_email::UserEmail;

pub struct ResetPasswordCommand {
    pub email: UserEmail,
    pub new_password: PlainPassword,
    pub reset_token: ResetToken,
}
