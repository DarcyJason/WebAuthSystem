use crate::domain::auth::value_objects::{
    credentials::plain_password::PlainPassword, user::user_email::UserEmail,
};

pub struct ChangePasswordCommand {
    pub email: UserEmail,
    pub new_password: PlainPassword,
}
