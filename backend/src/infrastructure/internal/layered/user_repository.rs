use crate::domain::error::DomainResult;
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
use crate::infrastructure::internal::caches::moka::user_repository::MokaUserRepository;
use crate::infrastructure::internal::caches::redis::user_repository::RedisUserRepository;
use crate::infrastructure::internal::persistence::postgres::user_repository::PostgresUserRepository;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct LayeredUserRepository {
    l1_cache: MokaUserRepository,
    l2_cache: RedisUserRepository,
    source_repo: PostgresUserRepository,
}

impl LayeredUserRepository {
    pub fn new(
        l1_cache: MokaUserRepository,
        l2_cache: RedisUserRepository,
        source_repo: PostgresUserRepository,
    ) -> Self {
        LayeredUserRepository {
            l1_cache,
            l2_cache,
            source_repo,
        }
    }

    async fn warm_up_l1(&self, user: &UserEntity) {
        let _ = self.l1_cache.save_with_ttl(user, self.l1_cache.ttl()).await;
    }

    async fn warm_up_l2_and_l1(&self, user: &UserEntity) {
        let _ = self.l2_cache.save_with_ttl(user, self.l2_cache.ttl()).await;
        self.warm_up_l1(user).await;
    }
}

#[async_trait]
impl UserCommandRepository for LayeredUserRepository {
    async fn save(&self, user: &UserEntity) -> DomainResult<UserEntity> {
        let saved = self.source_repo.save(user).await?;
        self.warm_up_l2_and_l1(&saved).await;
        Ok(saved)
    }
    async fn update_status(
        &self,
        user_id: &UserId,
        user_status: &UserStatus,
    ) -> DomainResult<UserEntity> {
        let updated = self.source_repo.update_status(user_id, user_status).await?;
        self.warm_up_l2_and_l1(&updated).await;
        Ok(updated)
    }
    async fn update_password_credential(
        &self,
        user_id: &UserId,
        user_password_credential: &PasswordCredential,
    ) -> DomainResult<UserEntity> {
        let updated = self
            .source_repo
            .update_password_credential(user_id, user_password_credential)
            .await?;
        self.warm_up_l2_and_l1(&updated).await;
        Ok(updated)
    }
    async fn update_access_token_version(
        &self,
        user_id: &UserId,
        access_token_version: &AccessTokenVersion,
    ) -> DomainResult<UserEntity> {
        let updated = self
            .source_repo
            .update_access_token_version(user_id, access_token_version)
            .await?;
        self.warm_up_l2_and_l1(&updated).await;
        Ok(updated)
    }
}

#[async_trait]
impl UserQueryRepository for LayeredUserRepository {
    async fn get_by_id(&self, user_id: &UserId) -> DomainResult<Option<UserEntity>> {
        if let Ok(Some(user)) = self.l1_cache.get_by_id(user_id).await {
            return Ok(Some(user));
        }
        if let Ok(Some(user)) = self.l2_cache.get_by_id(user_id).await {
            self.warm_up_l1(&user).await;
            return Ok(Some(user));
        }
        let user = self.source_repo.get_by_id(user_id).await?;
        if let Some(ref user) = user {
            self.warm_up_l2_and_l1(user).await;
        }
        Ok(user)
    }
    async fn get_by_name(&self, user_name: &UserName) -> DomainResult<Option<UserEntity>> {
        if let Ok(Some(user)) = self.l1_cache.get_by_name(user_name).await {
            return Ok(Some(user));
        }
        if let Ok(Some(user)) = self.l2_cache.get_by_name(user_name).await {
            self.warm_up_l1(&user).await;
            return Ok(Some(user));
        }
        let user = self.source_repo.get_by_name(user_name).await?;
        if let Some(ref user) = user {
            self.warm_up_l2_and_l1(user).await;
        }
        Ok(user)
    }
    async fn get_by_email(&self, user_email: &UserEmail) -> DomainResult<Option<UserEntity>> {
        if let Ok(Some(user)) = self.l1_cache.get_by_email(user_email).await {
            return Ok(Some(user));
        }
        if let Ok(Some(user)) = self.l2_cache.get_by_email(user_email).await {
            self.warm_up_l1(&user).await;
            return Ok(Some(user));
        }
        let user = self.source_repo.get_by_email(user_email).await?;
        if let Some(ref user) = user {
            self.warm_up_l2_and_l1(user).await;
        }
        Ok(user)
    }
    async fn get_by_name_or_email(
        &self,
        user_name: &Option<UserName>,
        user_email: &Option<UserEmail>,
    ) -> DomainResult<Option<UserEntity>> {
        if let Ok(Some(user)) = self
            .l1_cache
            .get_by_name_or_email(user_name, user_email)
            .await
        {
            return Ok(Some(user));
        }
        if let Ok(Some(user)) = self
            .l2_cache
            .get_by_name_or_email(user_name, user_email)
            .await
        {
            self.warm_up_l1(&user).await;
            return Ok(Some(user));
        }
        let user = self
            .source_repo
            .get_by_name_or_email(user_name, user_email)
            .await?;
        if let Some(ref user) = user {
            self.warm_up_l2_and_l1(user).await;
        }
        Ok(user)
    }
}
