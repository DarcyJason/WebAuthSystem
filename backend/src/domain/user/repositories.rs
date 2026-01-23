use async_trait::async_trait;
use surrealdb::RecordId;

use crate::domain::user::entities::User;
use crate::domain::user::value_objects::email::Email;
use crate::domain::user::value_objects::hash_password::HashPassword;
use crate::domain::user::value_objects::username::Username;
use crate::infrastructure::errors::InfraResult;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> InfraResult<Option<User>>;
    async fn find_by_id(&self, id: &RecordId) -> InfraResult<Option<User>>;
    async fn find_by_username(&self, username: &Username) -> InfraResult<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> InfraResult<Option<User>>;
    async fn find_by_username_or_email(
        &self,
        username: &Username,
        email: &Email,
    ) -> InfraResult<Option<User>>;
}

pub trait UserCache: Send + Sync {}
