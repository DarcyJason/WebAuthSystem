use crate::domain::auth::errors::credentials::login_identity_error::LoginIdentityError;
use crate::domain::auth::value_objects::user::user_email::UserEmail;
use crate::domain::auth::value_objects::user::user_name::UserName;
use serde::Deserialize;

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
                UserEmail::new(raw).map_err(|_| LoginIdentityError::UserEmailInvalid)?,
            ))
        } else {
            Ok(Self::UserName(
                UserName::new(raw).map_err(|_| LoginIdentityError::UserNameInvalid)?,
            ))
        }
    }
}
