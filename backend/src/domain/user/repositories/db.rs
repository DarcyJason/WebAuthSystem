use crate::domain::errors::{DomainError, DomainResult};
use crate::domain::user::entities::User;
use crate::domain::user::errors::UserError;
use crate::domain::user::value_objects::email::Email;
use crate::domain::user::value_objects::hash_password::HashPassword;
use crate::domain::user::value_objects::username::Username;
use crate::infrastructure::persistence::surreal::errors::SurrealDBError;
use crate::infrastructure::persistence::surreal::user_repository::SurrealUserRepository;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn save(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> DomainResult<Option<User>>;
    async fn find_by_id(&self, id: &str) -> DomainResult<Option<User>>;
    async fn find_by_username(&self, username: &Username) -> DomainResult<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> DomainResult<Option<User>>;
    async fn find_by_username_or_email(
        &self,
        username: &Username,
        email: &Email,
    ) -> DomainResult<Option<User>>;
}

pub struct SurrealUserRepositoryAdapter {
    inner: SurrealUserRepository,
}

impl SurrealUserRepositoryAdapter {
    pub fn new(inner: SurrealUserRepository) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl UserRepository for SurrealUserRepositoryAdapter {
    async fn save(
        &self,
        username: Username,
        email: Email,
        hash_password: HashPassword,
    ) -> DomainResult<Option<User>> {
        self.inner
            .save(username, email, hash_password)
            .await
            .map_err(|e| match e {
                SurrealDBError::ParseRecordToUserError => {
                    DomainError::UserError(UserError::CreateUserFailed)
                }
                _ => DomainError::DBUnavailable,
            })
    }
    async fn find_by_id(&self, id: &str) -> DomainResult<Option<User>> {
        self.inner.find_by_id(id).await.map_err(|e| match e {
            SurrealDBError::ParseRecordToUserError => {
                DomainError::UserError(UserError::UserNotFound)
            }
            _ => DomainError::DBUnavailable,
        })
    }
    async fn find_by_username(&self, username: &Username) -> DomainResult<Option<User>> {
        self.inner
            .find_by_username(username)
            .await
            .map_err(|e| match e {
                SurrealDBError::ParseRecordToUserError => {
                    DomainError::UserError(UserError::UserNotFound)
                }
                _ => DomainError::DBUnavailable,
            })
    }
    async fn find_by_email(&self, email: &Email) -> DomainResult<Option<User>> {
        self.inner.find_by_email(email).await.map_err(|e| match e {
            SurrealDBError::ParseRecordToUserError => {
                DomainError::UserError(UserError::UserNotFound)
            }
            _ => DomainError::DBUnavailable,
        })
    }
    async fn find_by_username_or_email(
        &self,
        username: &Username,
        email: &Email,
    ) -> DomainResult<Option<User>> {
        self.inner
            .find_by_username_or_email(username, email)
            .await
            .map_err(|e| match e {
                SurrealDBError::ParseRecordToUserError => {
                    DomainError::UserError(UserError::UserNotFound)
                }
                _ => DomainError::DBUnavailable,
            })
    }
}
