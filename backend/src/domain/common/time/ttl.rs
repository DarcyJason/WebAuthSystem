use std::{fmt::Display, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTL(Duration);

impl TTL {
    pub fn from_seonds(seconds: u64) -> Self {
        TTL(Duration::from_secs(seconds))
    }
    pub fn value(&self) -> &Duration {
        &self.0
    }
}

impl Display for TTL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_secs())
    }
}
