use crate::domain::common::time::timestamp::Timestamp;
use crate::domain::user::value_objects::user_email::UserEmail;
use crate::domain::user::value_objects::user_id::UserId;
use crate::domain::user::value_objects::user_name::UserName;
use crate::domain::user::value_objects::user_password_hash::UserPasswordHash;
use crate::domain::user::value_objects::user_status::UserStatus;
use serde::{Deserialize, Serialize};

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
}
