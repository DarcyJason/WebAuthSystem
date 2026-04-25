use crate::domain::user::value_objects::credential::plain_password::PlainPassword;

pub struct ResetPasswordCommand {
    pub token: String,
    pub new_password: PlainPassword,
}
