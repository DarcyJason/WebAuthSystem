use crate::domain::auth::value_objects::credentials::plain_password::PlainPassword;
use crate::domain::auth::value_objects::tokens::reset_token::ResetToken;
use crate::domain::user::entities::user::user_email::UserEmail;

pub struct ResetPasswordCommand {
    pub email: UserEmail,
    pub new_password: PlainPassword,
    pub reset_token: ResetToken,
}
