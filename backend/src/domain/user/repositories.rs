use async_trait::async_trait;
use surrealdb::RecordId;

use crate::domain::user::entities::User;
use crate::domain::user::value_objects::{Email, Username};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<Option<User>, anyhow::Error>;
    async fn find_by_id(&self, id: &RecordId) -> Result<Option<User>, anyhow::Error>;
    async fn find_by_username(&self, username: &Username) -> Result<Option<User>, anyhow::Error>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, anyhow::Error>;
}

pub trait UserCache: Send + Sync {}
