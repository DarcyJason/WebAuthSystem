use crate::domain::common::value_objects::time::ttl::TTL;
use crate::domain::error::{DomainResult, UserNotFoundSnafu, UserRepositoryJsonSnafu};
use crate::domain::user::aggregates::user::UserEntity;
use crate::domain::user::repositories::user_repository::{
    UserCommandRepository, UserQueryRepository,
};
use crate::domain::user::value_objects::access_token_version::AccessTokenVersion;
use crate::domain::user::value_objects::credential::password_credential::PasswordCredential;
use crate::domain::user::value_objects::user::user_email::UserEmail;
use crate::domain::user::value_objects::user::user_id::UserId;
use crate::domain::user::value_objects::user::user_name::UserName;
use crate::domain::user::value_objects::user::user_status::UserStatus;
use crate::infrastructure::internal::layered::cache_operation::CacheOperation;
use crate::infrastructure::internal::layered::cache_store::CacheStore;
use async_trait::async_trait;
use snafu::ResultExt;

#[derive(Debug, Clone)]
pub struct CacheUserRepository<S> {
    store: S,
    ttl: Option<TTL>,
}

impl<S: CacheStore> CacheUserRepository<S> {
    pub fn new(store: S) -> Self {
        CacheUserRepository { store, ttl: None }
    }
    pub fn with_ttl(store: S, ttl: TTL) -> Self {
        CacheUserRepository {
            store,
            ttl: Some(ttl),
        }
    }
    pub fn ttl(&self) -> Option<&TTL> {
        self.ttl.as_ref()
    }
    pub async fn get_by_any_key(&self, key: &str) -> DomainResult<Option<UserEntity>> {
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
    pub async fn get_by_id_key(&self, user_id: &UserId, id_key: &str) -> DomainResult<UserEntity> {
        self.get_by_any_key(id_key).await?.ok_or_else(|| {
            UserNotFoundSnafu {
                user_id: user_id.value().to_owned(),
            }
            .build()
        })
    }
    pub async fn serialize(
        &self,
        user: &UserEntity,
        operation: CacheOperation,
    ) -> DomainResult<String> {
        serde_json::to_string(user).context(UserRepositoryJsonSnafu {
            layer: self.store.layer(),
            operation,
            message: "serialize user failed".to_string(),
        })
    }
    pub async fn save_with_ttl(
        &self,
        user: &UserEntity,
        ttl: Option<&TTL>,
    ) -> DomainResult<UserEntity> {
        let id_key = format!("user:id:{}", user.id().value());
        let payload = self.serialize(user, CacheOperation::Serialize).await?;
        self.save_all_keys(user, payload, ttl).await?;
        self.get_by_id_key(&user.id(), &id_key).await
    }
    pub async fn save_all_keys(
        &self,
        user: &UserEntity,
        payload: String,
        ttl: Option<&TTL>,
    ) -> DomainResult<()> {
        self.store
            .batch_set(
                vec![
                    (format!("user:id:{}", user.id().value()), payload.clone()),
                    (
                        format!("user:email:{}", user.email().value()),
                        payload.clone(),
                    ),
                    (format!("user:name:{}", user.name().value()), payload),
                ],
                ttl,
            )
            .await
    }
}

#[async_trait]
impl<S: CacheStore> UserCommandRepository for CacheUserRepository<S> {
    async fn save(&self, user: &UserEntity) -> DomainResult<UserEntity> {
        let id_key = format!("user:id:{}", user.id().value());
        let payload = self.serialize(user, CacheOperation::Serialize).await?;
        self.save_all_keys(user, payload, self.ttl.as_ref()).await?;
        self.get_by_id_key(&user.id(), &id_key).await
    }
    async fn update_status(
        &self,
        user_id: &UserId,
        user_status: &UserStatus,
    ) -> DomainResult<UserEntity> {
        let id_key = format!("user:id:{}", user_id.value());
        let mut user = self.get_by_id_key(user_id, &id_key).await?;
        user.update_status(user_status);
        let payload = self.serialize(&user, CacheOperation::Set).await?;
        self.save_all_keys(&user, payload, self.ttl.as_ref())
            .await?;
        self.get_by_id_key(&user.id(), &id_key).await
    }
    async fn update_password_credential(
        &self,
        user_id: &UserId,
        user_password_credential: &PasswordCredential,
    ) -> DomainResult<UserEntity> {
        let id_key = format!("user:id:{}", user_id.value());
        let mut user = self.get_by_id_key(user_id, &id_key).await?;
        user.update_password_credential(user_password_credential);
        let payload = self.serialize(&user, CacheOperation::Set).await?;
        self.save_all_keys(&user, payload, self.ttl.as_ref())
            .await?;
        self.get_by_id_key(&user.id(), &id_key).await
    }
    async fn update_access_token_version(
        &self,
        user_id: &UserId,
        access_token_version: &AccessTokenVersion,
    ) -> DomainResult<UserEntity> {
        let id_key = format!("user:id:{}", user_id.value());
        let mut user = self.get_by_id_key(user_id, &id_key).await?;
        user.update_access_token_version(access_token_version);
        let payload = self.serialize(&user, CacheOperation::Set).await?;
        self.save_all_keys(&user, payload, self.ttl.as_ref())
            .await?;
        self.get_by_id_key(&user.id(), &id_key).await
    }
}

#[async_trait]
impl<S: CacheStore> UserQueryRepository for CacheUserRepository<S> {
    async fn get_by_id(&self, user_id: &UserId) -> DomainResult<Option<UserEntity>> {
        self.get_by_any_key(&format!("user:id:{}", user_id.value()))
            .await
    }
    async fn get_by_name(&self, user_name: &UserName) -> DomainResult<Option<UserEntity>> {
        self.get_by_any_key(&format!("user:name:{}", user_name.value()))
            .await
    }
    async fn get_by_email(&self, user_email: &UserEmail) -> DomainResult<Option<UserEntity>> {
        self.get_by_any_key(&format!("user:email:{}", user_email.value()))
            .await
    }
    async fn get_by_name_or_email(
        &self,
        user_name: &Option<UserName>,
        user_email: &Option<UserEmail>,
    ) -> DomainResult<Option<UserEntity>> {
        match (user_name, user_email) {
            (Some(user_name), None) => {
                let existing_user = self
                    .get_by_any_key(&format!("user:name:{}", user_name.value()))
                    .await?;
                if existing_user.is_some() {
                    Ok(existing_user)
                } else {
                    Ok(None)
                }
            }
            (None, Some(user_email)) => {
                let existing_user = self
                    .get_by_any_key(&format!("user:email:{}", user_email.value()))
                    .await?;
                if existing_user.is_some() {
                    Ok(existing_user)
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }
}
