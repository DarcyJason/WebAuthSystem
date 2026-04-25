use crate::domain::error::{DomainResult, InvalidPlainPasswordSnafu};
use once_cell::sync::Lazy;
use regex::Regex;
use snafu::ensure;

const PLAIN_PASSWORD_MIN_LENGTH: usize = 8;
const PLAIN_PASSWORD_MAX_LENGTH: usize = 128;
static UPPERCASE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]").unwrap());
static LOWERCASE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-z]").unwrap());
static NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]").unwrap());
static SPECIAL_CHAR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^A-Za-z0-9]").unwrap());

#[derive(Debug, Clone, PartialEq)]
pub struct PlainPassword(String);

impl PlainPassword {
    pub fn new(plain_password: impl Into<String>) -> DomainResult<Self> {
        let plain_password = plain_password.into();
        ensure!(
            plain_password.len() != 0,
            InvalidPlainPasswordSnafu {
                password: plain_password.clone(),
                message: "password is required".to_string()
            }
        );
        ensure!(
            plain_password.len() >= PLAIN_PASSWORD_MIN_LENGTH,
            InvalidPlainPasswordSnafu {
                password: plain_password.clone(),
                message: format!(
                    "password must be at least {} characters",
                    PLAIN_PASSWORD_MIN_LENGTH
                )
            }
        );
        ensure!(
            plain_password.len() <= PLAIN_PASSWORD_MAX_LENGTH,
            InvalidPlainPasswordSnafu {
                password: plain_password.clone(),
                message: format!(
                    "password is too long (max {} chars)",
                    PLAIN_PASSWORD_MAX_LENGTH
                )
            }
        );
        ensure!(
            UPPERCASE_REGEX.is_match(&plain_password),
            InvalidPlainPasswordSnafu {
                password: plain_password.clone(),
                message: "password must contain at least one uppercase letter".to_string()
            }
        );
        ensure!(
            LOWERCASE_REGEX.is_match(&plain_password),
            InvalidPlainPasswordSnafu {
                password: plain_password.clone(),
                message: "password must contain at least one lowercase letter".to_string()
            }
        );
        ensure!(
            NUMBER_REGEX.is_match(&plain_password),
            InvalidPlainPasswordSnafu {
                password: plain_password.clone(),
                message: "password must contain at least one number".to_string()
            }
        );
        ensure!(
            SPECIAL_CHAR_REGEX.is_match(&plain_password),
            InvalidPlainPasswordSnafu {
                password: plain_password,
                message: "password must contain at least one special character".to_string()
            }
        );
        Ok(Self(plain_password.into()))
    }

    pub fn value(&self) -> String {
        self.0.to_owned()
    }
}
