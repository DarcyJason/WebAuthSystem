use serde::Deserialize;

use crate::domain::user::value_objects::{
    email::Email, hash_password::HashPassword, username::Username,
};

#[derive(Debug, Deserialize)]
pub struct RegisterCommand {
    pub username: Username,
    pub email: Email,
    pub hash_password: HashPassword,
}
