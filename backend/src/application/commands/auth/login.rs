use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginCommand {
    pub username_or_email: String,
    pub password: String,
}
