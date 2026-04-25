use crate::domain::error::{DomainResult, InvalidUserEmailSnafu};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use snafu::ensure;
use std::fmt::Display;

const USER_EMAIL_MIN_LENGTH: usize = 5;
const USER_EMAIL_MAX_LENGTH: usize = 100;
static USER_EMAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap());

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
            user_email.len() >= USER_EMAIL_MIN_LENGTH,
            InvalidUserEmailSnafu {
                user_email,
                message: format!("user email too short (min {} chars)", USER_EMAIL_MIN_LENGTH)
            }
        );
        ensure!(
            user_email.len() <= USER_EMAIL_MAX_LENGTH,
            InvalidUserEmailSnafu {
                user_email,
                message: format!("user email too long (max {} chars)", USER_EMAIL_MAX_LENGTH)
            }
        );
        ensure!(
            USER_EMAIL_REGEX.is_match(&user_email),
            InvalidUserEmailSnafu {
                user_email,
                message: "user email format is invalid".to_string()
            }
        );
        Ok(Self(user_email))
    }
    pub fn value(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for UserEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
