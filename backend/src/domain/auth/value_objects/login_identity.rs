use serde::Deserialize;

use crate::domain::user::value_objects::{
    email::{Email, EmailError},
    useranme::{Username, UsernameError},
};

pub enum LoginIdentityError {
    UsernameError(UsernameError),
    EmailError(EmailError),
}

#[derive(Debug, Clone, Deserialize)]
pub enum LoginIdentity {
    Username(Username),
    Email(Email),
}

impl LoginIdentity {
    pub fn parse(raw: String) -> Result<Self, LoginIdentityError> {
        if raw.contains("@") {
            Ok(Self::Email(
                Email::new(raw).map_err(|e| LoginIdentityError::EmailError(e))?,
            ))
        } else {
            Ok(Self::Username(
                Username::new(raw).map_err(|e| LoginIdentityError::UsernameError(e))?,
            ))
        }
    }
}
