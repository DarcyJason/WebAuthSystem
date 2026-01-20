use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::domain::user::value_objects::{
    email::Email, hash_password::HashPassword, username::Username,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: RecordId,
    username: Username,
    email: Email,
    hash_password: HashPassword,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn id(&self) -> &RecordId {
        &self.id
    }
    pub fn username(&self) -> &Username {
        &self.username
    }
    pub fn email(&self) -> &Email {
        &self.email
    }
    pub fn hash_password(&self) -> &HashPassword {
        &self.hash_password
    }
    pub fn created_at(&self) -> &Option<DateTime<Utc>> {
        &self.created_at
    }
    pub fn updated_at(&self) -> &Option<DateTime<Utc>> {
        &self.updated_at
    }
}
