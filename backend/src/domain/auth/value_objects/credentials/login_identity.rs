use crate::domain::auth::value_objects::user::user_email::{UserEmail, UserEmailError};
use crate::domain::auth::value_objects::user::user_name::{UserName, UserNameError};
use serde::Deserialize;

pub enum LoginIdentityError {
    UserNameError(UserNameError),
    UserEmailError(UserEmailError),
    LoginIdentityRequired,
}

#[derive(Debug, Clone, Deserialize)]
pub enum LoginIdentity {
    UserName(UserName),
    UserEmail(UserEmail),
}

impl LoginIdentity {
    pub fn parse(raw: String) -> Result<Self, LoginIdentityError> {
        if raw.is_empty() {
            return Err(LoginIdentityError::LoginIdentityRequired);
        }
        if raw.contains("@") {
            Ok(Self::UserEmail(
                UserEmail::new(raw).map_err(LoginIdentityError::UserEmailError)?,
            ))
        } else {
            Ok(Self::UserName(
                UserName::new(raw).map_err(LoginIdentityError::UserNameError)?,
            ))
        }
    }
}
