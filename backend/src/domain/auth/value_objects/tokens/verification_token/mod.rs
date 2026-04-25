pub mod verification_token_kind;
pub mod verification_token_value;
pub mod verification_used;

use crate::domain::auth::value_objects::tokens::verification_token::verification_token_kind::VerificationTokenKind;
use crate::domain::auth::value_objects::tokens::verification_token::verification_token_value::VerificationTokenValue;
use crate::domain::auth::value_objects::tokens::verification_token::verification_used::VerificationTokenUsed;
use crate::domain::common::value_objects::time::time_stamp::Timestamp;
use crate::domain::user::value_objects::user::user_id::UserId;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationToken {
    value: VerificationTokenValue,
    user_id: UserId,
    kind: VerificationTokenKind,
    used: VerificationTokenUsed,
    created_at: Timestamp,
    expires_at: Timestamp,
}

impl VerificationToken {
    pub fn issue(user_id: UserId, kind: VerificationTokenKind, expires_in_seconds: i64) -> Self {
        let now = Utc::now();
        Self {
            value: VerificationTokenValue::new(),
            user_id,
            kind,
            used: VerificationTokenUsed::new(),
            created_at: Timestamp::new(now),
            expires_at: Timestamp::new(now + Duration::seconds(expires_in_seconds)),
        }
    }

    pub fn from_parts(
        value: VerificationTokenValue,
        user_id: UserId,
        kind: VerificationTokenKind,
        used: VerificationTokenUsed,
        created_at: Timestamp,
        expires_at: Timestamp,
    ) -> Self {
        Self {
            value,
            user_id,
            kind,
            used,
            created_at,
            expires_at,
        }
    }

    pub fn value(&self) -> VerificationTokenValue {
        self.value.to_owned()
    }
    pub fn user_id(&self) -> UserId {
        self.user_id.to_owned()
    }
    pub fn kind(&self) -> VerificationTokenKind {
        self.kind.to_owned()
    }
    pub fn used(&self) -> VerificationTokenUsed {
        self.used.to_owned()
    }
    pub fn created_at(&self) -> Timestamp {
        self.created_at.to_owned()
    }
    pub fn expires_at(&self) -> Timestamp {
        self.expires_at.to_owned()
    }
}
