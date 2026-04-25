use crate::domain::user::value_objects::credential::plain_password::PlainPassword;
use crate::domain::user::value_objects::user::user_id::UserId;

pub struct ChangePasswordCommand {
    pub user_id: UserId,
    pub current_password: PlainPassword,
    pub new_password: PlainPassword,
}
