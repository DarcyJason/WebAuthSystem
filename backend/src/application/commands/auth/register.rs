use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterCommand {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}
