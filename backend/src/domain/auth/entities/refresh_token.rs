use crate::domain::auth::value_objects::tokens::refresh_token::RefreshToken;
use crate::domain::auth::value_objects::tokens::refresh_token_hash::RefreshTokenHash;
use crate::domain::auth::value_objects::tokens::refresh_token_id::RefreshTokenId;
use crate::domain::common::value_objects::time::time_stamp::Timestamp;
use crate::domain::user::value_objects::user::user_id::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenEntity {
    id: RefreshTokenId,
    user_id: UserId,
    token_hash: RefreshTokenHash,
    expires_at: Timestamp,
    created_at: Timestamp,
}

impl RefreshTokenEntity {
    pub fn issue(user_id: UserId, token: &RefreshToken, expires_at: Timestamp) -> Self {
        let id = RefreshTokenId::new();
        let token_hash = RefreshTokenHash::from_refresh_token(token);
        let created_at = Timestamp::now();
        Self {
            id,
            user_id,
            token_hash,
            expires_at,
            created_at,
        }
    }

    pub fn from_parts(
        id: RefreshTokenId,
        user_id: UserId,
        token_hash: RefreshTokenHash,
        expires_at: Timestamp,
        created_at: Timestamp,
    ) -> Self {
        Self {
            id,
            user_id,
            token_hash,
            expires_at,
            created_at,
        }
    }

    pub fn id(&self) -> &RefreshTokenId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn token_hash(&self) -> &RefreshTokenHash {
        &self.token_hash
    }

    pub fn expires_at(&self) -> &Timestamp {
        &self.expires_at
    }

    pub fn created_at(&self) -> &Timestamp {
        &self.created_at
    }
}
