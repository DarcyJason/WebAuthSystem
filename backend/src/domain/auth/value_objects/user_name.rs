use serde::{Deserialize, Serialize};

pub enum UserNameError {
    UserNameInvalid,
    UserNameTooLong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserName(String);

impl UserName {
    pub fn new(user_name: impl Into<String>) -> Result<Self, UserNameError> {
        let user_name: String = user_name.into();
        if user_name.contains("@") {
            return Err(UserNameError::UserNameInvalid);
        }
        if user_name.len() > 20 {
            return Err(UserNameError::UserNameTooLong);
        }
        Ok(UserName(user_name))
    }
}
