use serde::Deserialize;

use crate::domain::{
    auth::value_objects::plain_password::PlainPassword,
    user::value_objects::{email::Email, username::Username},
};

#[derive(Debug, Deserialize)]
pub struct RegisterCommand {
    pub username: Username,
    pub email: Email,
    pub password: PlainPassword,
}
