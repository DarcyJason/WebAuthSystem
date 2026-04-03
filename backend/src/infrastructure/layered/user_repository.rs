use crate::domain::error::DomainResult;
use crate::domain::identities::aggregates::user::User;
use crate::domain::identities::repositories::user_repository::UserRepository;
use crate::domain::identities::value_objects::credential::password_credential::PasswordCredential;
use crate::domain::identities::value_objects::user::user_email::UserEmail;
use crate::domain::identities::value_objects::user::user_id::UserId;
use crate::domain::identities::value_objects::user::user_name::UserName;
use crate::domain::identities::value_objects::user::user_status::UserStatus;
use async_trait::async_trait;
use std::sync::Arc;

pub struct LayeredUserRepository {
    l1_cache: Arc<dyn UserRepository>,
    l2_cache: Arc<dyn UserRepository>,
    source_repo: Arc<dyn UserRepository>,
}

impl LayeredUserRepository {
    pub fn new(
        l1_cache: Arc<dyn UserRepository>,
        l2_cache: Arc<dyn UserRepository>,
        source_repo: Arc<dyn UserRepository>,
    ) -> Self {
        LayeredUserRepository {
            l1_cache,
            l2_cache,
            source_repo,
        }
    }

    async fn warm_up_l1(&self, user: &User) {
        let _ = self.l1_cache.save(user).await;
    }

    async fn warm_up_l2_and_l1(&self, user: &User) {
        let _ = self.l2_cache.save(user).await;
        self.warm_up_l1(user).await;
    }
}

#[async_trait]
impl UserRepository for LayeredUserRepository {
    async fn save(&self, user: &User) -> DomainResult<User> {
        let saved = self.source_repo.save(user).await?;
        self.warm_up_l2_and_l1(&saved).await;
        Ok(saved)
    }
    async fn get_by_id(&self, user_id: &UserId) -> DomainResult<Option<User>> {
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
    async fn get_by_name(&self, user_name: &UserName) -> DomainResult<Option<User>> {
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
    async fn get_by_email(&self, user_email: &UserEmail) -> DomainResult<Option<User>> {
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
    async fn update_status(
        &self,
        user_id: &UserId,
        user_status: &UserStatus,
    ) -> DomainResult<User> {
        let updated = self.source_repo.update_status(user_id, user_status).await?;
        self.warm_up_l2_and_l1(&updated).await;
        Ok(updated)
    }
    async fn update_password_credential(
        &self,
        user_id: &UserId,
        user_password_credential: &PasswordCredential,
    ) -> DomainResult<User> {
        let updated = self
            .source_repo
            .update_password_credential(user_id, user_password_credential)
            .await?;
        self.warm_up_l2_and_l1(&updated).await;
        Ok(updated)
    }
}
