use crate::domain::auth::value_objects::user::user_email::UserEmail;
use crate::domain::auth::value_objects::user::user_id::UserId;
use crate::domain::auth::value_objects::user::user_name::UserName;
use crate::domain::auth::value_objects::user::user_password_hash::UserPasswordHash;
use crate::domain::auth::value_objects::user::user_status::UserStatus;
use crate::domain::common::time::timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    name: UserName,
    email: UserEmail,
    password_hash: UserPasswordHash,
    status: UserStatus,
    created_at: Timestamp,
    updated_at: Timestamp,
}

impl User {
    pub fn new(name: UserName, email: UserEmail, password_hash: UserPasswordHash) -> User {
        let user_id = UserId::new();
        let status = UserStatus::new(false);
        let now = Timestamp::now();
        let created_at = now.clone();
        let updated_at = now;
        User {
            id: user_id,
            name,
            email,
            password_hash,
            status,
            created_at,
            updated_at,
        }
    }
    pub fn id(&self) -> &UserId {
        &self.id
    }
    pub fn name(&self) -> &UserName {
        &self.name
    }
    pub fn email(&self) -> &UserEmail {
        &self.email
    }
    pub fn password_hash(&self) -> &UserPasswordHash {
        &self.password_hash
    }
    pub fn status(&self) -> &UserStatus {
        &self.status
    }
    pub fn created_at(&self) -> &Timestamp {
        &self.created_at
    }
    pub fn updated_at(&self) -> &Timestamp {
        &self.updated_at
    }

    pub fn to_redis_value(&self) -> Result<String, SerdeJsonError> {
        serde_json::to_string(self)
    }

    pub fn from_redis_value(redis_value: String) -> Result<Self, SerdeJsonError> {
        serde_json::from_str(&redis_value)
    }

    pub fn from_redis_optional_value(
        redis_value: Option<String>,
    ) -> Result<Option<Self>, SerdeJsonError> {
        match redis_value {
            Some(value) => Self::from_redis_value(value).map(Some),
            None => Ok(None),
        }
    }

    pub fn mark_as_verified(&mut self) {
        self.status = UserStatus::new(true);
        self.updated_at = Timestamp::now();
    }
}
