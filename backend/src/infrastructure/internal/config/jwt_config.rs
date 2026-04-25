use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expires_in_seconds: i64,
    pub refresh_token_expires_in_days: i64,
    pub email_verify_expires_in_seconds: i64,
    pub password_reset_expires_in_seconds: i64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        JwtConfig {
            secret: "your_jwt_secret".to_string(),
            expires_in_seconds: 3600,
            refresh_token_expires_in_days: 7,
            email_verify_expires_in_seconds: 86400,
            password_reset_expires_in_seconds: 3600,
        }
    }
}
