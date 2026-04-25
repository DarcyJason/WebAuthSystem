use crate::domain::auth::value_objects::tokens::refresh_token::RefreshToken;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenHash(String);

impl RefreshTokenHash {
    pub fn from_refresh_token(token: &RefreshToken) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(token.value().as_bytes());
        Self(format!("{:x}", hasher.finalize()))
    }

    pub fn from_str(hash: impl Into<String>) -> Self {
        Self(hash.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
