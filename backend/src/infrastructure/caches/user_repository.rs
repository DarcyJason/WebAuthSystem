use crate::domain::error::{DomainResult, UserNotFoundSnafu, UserRepositoryJsonSnafu};
use crate::domain::identities::aggregates::user::User;
use crate::domain::identities::repositories::user_repository::UserRepository;
use crate::domain::identities::value_objects::credential::password_credential::PasswordCredential;
use crate::domain::identities::value_objects::user::user_email::UserEmail;
use crate::domain::identities::value_objects::user::user_id::UserId;
use crate::domain::identities::value_objects::user::user_name::UserName;
use crate::domain::identities::value_objects::user::user_status::UserStatus;
use crate::infrastructure::layered::cache_operation::CacheOperation;
use crate::infrastructure::layered::cache_store::CacheStore;
use async_trait::async_trait;
use snafu::ResultExt;

pub struct CacheUserRepository<S> {
    store: S,
}

impl<S: CacheStore> CacheUserRepository<S> {
    pub fn new(store: S) -> Self {
        CacheUserRepository { store }
    }
    pub async fn get_optional(&self, key: &str) -> DomainResult<Option<User>> {
        match self.store.get(key).await? {
            Some(data) => Ok(Some(serde_json::from_str(&data).context(
                UserRepositoryJsonSnafu {
                    layer: self.store.layer(),
                    operation: CacheOperation::Deserialize,
                    message: format!("deserialize user failed for key: {}", key),
                },
            )?)),
            None => Ok(None),
        }
    }
    pub async fn get_by_id_key(&self, user_id: &UserId, id_key: &str) -> DomainResult<User> {
        self.get_optional(id_key).await?.ok_or_else(|| {
            UserNotFoundSnafu {
                user_id: user_id.value().to_owned(),
            }
            .build()
        })
    }
    pub async fn serialize(&self, user: &User, operation: CacheOperation) -> DomainResult<String> {
        serde_json::to_string(user).context(UserRepositoryJsonSnafu {
            layer: self.store.layer(),
            operation,
            message: "serialize user failed".to_string(),
        })
    }
    pub async fn save_all_keys(&self, user: &User, payload: String) -> DomainResult<()> {
        self.store
            .batch_set(vec![
                (format!("user:id:{}", user.id().value()), payload.clone()),
                (
                    format!("user:email:{}", user.email().value()),
                    payload.clone(),
                ),
                (format!("user:name:{}", user.name().value()), payload),
            ])
            .await
    }
}

#[async_trait]
impl<S: CacheStore> UserRepository for CacheUserRepository<S> {
    async fn save(&self, user: &User) -> DomainResult<User> {
        let id_key = format!("user:id:{}", user.id().value());
        let payload = self.serialize(user, CacheOperation::Serialize).await?;
        self.save_all_keys(user, payload).await?;
        self.get_by_id_key(&user.id(), &id_key).await
    }
    async fn get_by_id(&self, user_id: &UserId) -> DomainResult<Option<User>> {
        self.get_optional(&format!("user:id:{}", user_id.value()))
            .await
    }
    async fn get_by_name(&self, user_name: &UserName) -> DomainResult<Option<User>> {
        self.get_optional(&format!("user:name:{}", user_name.value()))
            .await
    }
    async fn get_by_email(&self, user_email: &UserEmail) -> DomainResult<Option<User>> {
        self.get_optional(&format!("user:email:{}", user_email.value()))
            .await
    }
    async fn update_status(
        &self,
        user_id: &UserId,
        user_status: &UserStatus,
    ) -> DomainResult<User> {
        let id_key = format!("user:id:{}", user_id.value());
        let mut user = self.get_by_id_key(user_id, &id_key).await?;
        user.update_status(user_status);
        let payload = self.serialize(&user, CacheOperation::Set).await?;
        self.save_all_keys(&user, payload).await?;
        self.get_by_id_key(&user.id(), &id_key).await
    }
    async fn update_password_credential(
        &self,
        user_id: &UserId,
        user_password_credential: &PasswordCredential,
    ) -> DomainResult<User> {
        let id_key = format!("user:id:{}", user_id.value());
        let mut user = self.get_by_id_key(user_id, &id_key).await?;
        user.update_password_credential(user_password_credential);
        let payload = self.serialize(&user, CacheOperation::Set).await?;
        self.save_all_keys(&user, payload).await?;
        self.get_by_id_key(&user.id(), &id_key).await
    }
}
