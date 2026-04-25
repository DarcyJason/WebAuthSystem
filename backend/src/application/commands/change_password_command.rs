use crate::domain::user::value_objects::credential::plain_password::PlainPassword;

pub struct ChangePasswordCommand {
    pub current_password: PlainPassword,
    pub new_password: PlainPassword,
}
