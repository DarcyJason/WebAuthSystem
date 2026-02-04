use serde::{Deserialize, Serialize};

pub enum UserEmailError {
    UserEmailRequired,
    UserEmailInvalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn new(user_email: impl Into<String>) -> Result<Self, UserEmailError> {
        let user_email: String = user_email.into();
        if user_email.is_empty() {
            return Err(UserEmailError::UserEmailRequired);
        }
        if !user_email.contains("@") {
            return Err(UserEmailError::UserEmailInvalid);
        }
        Ok(UserEmail(user_email))
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
