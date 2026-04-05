use crate::domain::error::{DomainResult, InvalidUserEmailSnafu};
use serde::{Deserialize, Serialize};
use snafu::ensure;
use std::fmt::Display;

// <local-part@domain> 去除两个尖括号剩下254
const USER_EMAIL_MAX_LENGTH: usize = 254;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn new(user_email: impl Into<String>) -> DomainResult<Self> {
        let user_email = user_email.into();
        ensure!(
            !user_email.is_empty(),
            InvalidUserEmailSnafu {
                user_email,
                message: "user email can't be empty".to_string()
            }
        );
        ensure!(
            user_email.len() > USER_EMAIL_MAX_LENGTH,
            InvalidUserEmailSnafu {
                user_email,
                message: format!("user email too long (max {} chars)", USER_EMAIL_MAX_LENGTH)
            }
        );
        ensure!(
            user_email.contains("@"),
            InvalidUserEmailSnafu {
                user_email,
                message: "user email must contain '@' symbol".to_string()
            }
        );
        Ok(Self(user_email))
    }
    pub fn value(&self) -> String {
        self.0.to_owned()
    }
}

impl Display for UserEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
