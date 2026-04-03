use crate::domain::error::{DomainResult, InvalidUserNameSnafu};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use snafu::ensure;

static USER_NAME_LENGTH_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.{4,31}$").unwrap());
static USER_NAME_START_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_]").unwrap());
static USER_NAME_CHARS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_]+$").unwrap());

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
            USER_NAME_LENGTH_REGEX.is_match(&user_name),
            InvalidUserNameSnafu {
                user_name,
                message: "user name must be 4-31 charcters".to_string()
            }
        );
        ensure!(
            USER_NAME_START_REGEX.is_match(&user_name),
            InvalidUserNameSnafu {
                user_name,
                message: "user name must start with letter or underscore".to_string()
            }
        );
        ensure!(
            USER_NAME_CHARS_REGEX.is_match(&user_name),
            InvalidUserNameSnafu {
                user_name,
                message: "user name can only contain letters, numbers and underscore".to_string()
            }
        );
        Ok(UserName(user_name))
    }
    pub fn value(&self) -> String {
        self.0.to_owned()
    }
    pub fn update_name(&mut self, new_name: &UserName) {
        *self = new_name.to_owned();
    }
}
