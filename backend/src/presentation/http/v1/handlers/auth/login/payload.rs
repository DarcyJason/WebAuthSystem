use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username_or_email: String,
    pub password: String,
}
