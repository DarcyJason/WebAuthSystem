use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn new(datetime: DateTime<Utc>) -> Self {
        Self(datetime)
    }
    pub fn now() -> Self {
        Self(Utc::now())
    }
    pub fn value(&self) -> &DateTime<Utc> {
        &self.0
    }
}
