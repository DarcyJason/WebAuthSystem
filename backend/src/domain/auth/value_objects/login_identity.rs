use serde::Deserialize;

use crate::domain::user::value_objects::{
    email::{Email, EmailError},
    username::{Username, UsernameError},
};

pub enum LoginIdentityError {
    UsernameError(UsernameError),
    EmailError(EmailError),
    LoginIdentityRequired,
}

#[derive(Debug, Clone, Deserialize)]
pub enum LoginIdentity {
    Username(Username),
    Email(Email),
}

impl LoginIdentity {
    pub fn parse(raw: String) -> Result<Self, LoginIdentityError> {
        if raw.is_empty() {
            return Err(LoginIdentityError::LoginIdentityRequired);
        }
        if raw.contains("@") {
            Ok(Self::Email(
                Email::new(raw).map_err(LoginIdentityError::EmailError)?,
            ))
        } else {
            Ok(Self::Username(
                Username::new(raw).map_err(LoginIdentityError::UsernameError)?,
            ))
        }
    }
}
