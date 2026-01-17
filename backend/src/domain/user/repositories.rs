use async_trait::async_trait;
use surrealdb::RecordId;

use crate::domain::error::DomainResult;
use crate::domain::user::entities::User;
use crate::domain::user::value_objects::{Email, HashPassword, Username};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> DomainResult<Option<User>>;
    async fn find_by_id(&self, id: &RecordId) -> DomainResult<Option<User>>;
    async fn find_by_username(&self, username: &Username) -> DomainResult<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> DomainResult<Option<User>>;
}

pub trait UserCache: Send + Sync {}
