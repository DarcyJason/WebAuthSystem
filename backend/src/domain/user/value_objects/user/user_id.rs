use crate::domain::error::{DomainResult, InvalidUserIdSnafu};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn from_raw_user_id(user_id: impl Into<String>) -> DomainResult<Self> {
        let user_id = user_id.into();
        Ok(UserId(
            user_id
                .clone()
                .parse::<Uuid>()
                .context(InvalidUserIdSnafu { user_id })?,
        ))
    }
    pub fn value(&self) -> &Uuid {
        self.0.as_ref()
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
