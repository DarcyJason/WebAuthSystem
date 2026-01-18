use async_trait::async_trait;
use surrealdb::RecordId;

use crate::domain::error::RepoResult;
use crate::domain::user::entities::User;
use crate::domain::user::value_objects::{Email, HashPassword, Username};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> RepoResult<Option<User>>;
    async fn find_by_id(&self, id: &RecordId) -> RepoResult<Option<User>>;
    async fn find_by_username(&self, username: &Username) -> RepoResult<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> RepoResult<Option<User>>;
}

pub trait UserCache: Send + Sync {}
