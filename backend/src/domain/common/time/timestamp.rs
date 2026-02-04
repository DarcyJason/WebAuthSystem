use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn new(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }
    pub fn now() -> Self {
        Self(Utc::now())
    }
    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}
