use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionName(String);

impl SessionName {
    pub fn new(session_name: impl Into<String>) -> Self {
        SessionName(session_name.into())
    }
}
