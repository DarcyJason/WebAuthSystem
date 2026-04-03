use crate::domain::error::{DomainResult, InvalidUserIdSnafu};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn from_raw_user_id(user_id: String) -> DomainResult<Self> {
        Ok(UserId(
            user_id
                .parse::<Uuid>()
                .context(InvalidUserIdSnafu { user_id })?,
        ))
    }
    pub fn value(&self) -> Uuid {
        self.0.to_owned()
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}
