use crate::domain::{
    auth::value_objects::plain_password::PlainPassword,
    user::value_objects::{user_email::UserEmail, user_name::UserName},
};

pub struct RegisterCommand {
    pub name: UserName,
    pub email: UserEmail,
    pub plain_password: PlainPassword,
}
