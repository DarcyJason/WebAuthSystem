use crate::domain::error::{DomainResult, InvalidUserNameSnafu};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use snafu::ensure;

const USER_NAME_MIN_LENGTH: usize = 2;
const USER_NAME_MAX_LENGTH: usize = 50;
static USER_NAME_CHARS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z\s]*$").unwrap());

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserName(String);

impl UserName {
    pub fn new(user_name: impl Into<String>) -> DomainResult<Self> {
        let user_name = user_name.into();
        ensure!(
            !user_name.is_empty(),
            InvalidUserNameSnafu {
                user_name,
                message: "user name can't be empty".to_string()
            }
        );
        ensure!(
            user_name.len() >= USER_NAME_MIN_LENGTH,
            InvalidUserNameSnafu {
                user_name,
                message: format!(
                    "user name must be at least {} characters",
                    USER_NAME_MIN_LENGTH
                )
            }
        );
        ensure!(
            user_name.len() <= USER_NAME_MAX_LENGTH,
            InvalidUserNameSnafu {
                user_name,
                message: format!(
                    "user name must be less than {} characters",
                    USER_NAME_MAX_LENGTH
                )
            }
        );
        ensure!(
            USER_NAME_CHARS_REGEX.is_match(&user_name),
            InvalidUserNameSnafu {
                user_name,
                message: "user name can only contain letters and spaces".to_string()
            }
        );
        Ok(UserName(user_name))
    }
    pub fn value(&self) -> &str {
        self.0.as_ref()
    }
    pub fn update_name(&mut self, new_name: &UserName) {
        *self = new_name.to_owned();
    }
}
