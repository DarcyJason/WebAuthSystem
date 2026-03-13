use crate::domain::{
    auth::value_objects::credentials::plain_password::PlainPassword,
    user::entities::user::{user_email::UserEmail, user_name::UserName},
};

pub struct RegisterCommand {
    pub name: UserName,
    pub email: UserEmail,
    pub plain_password: PlainPassword,
}
