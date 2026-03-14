use crate::domain::auth::entity::user::User;
use crate::domain::auth::repository::user_repository::UserRepository;
use crate::domain::auth::value_objects::user::user_email::UserEmail;
use crate::domain::auth::value_objects::user::user_id::UserId;
use crate::domain::auth::value_objects::user::user_name::UserName;
use crate::infrastructure::errors::user_repository_error::UserRepositoryError;
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
        let _ = self.l1_cache.save(user.clone()).await;
    }

    async fn warm_up_l2_and_l1(&self, user: &User) {
        let _ = self.l2_cache.save(user.clone()).await;
        self.warm_up_l1(user).await;
    }
}

#[async_trait]
impl UserRepository for LayeredUserRepository {
    async fn save(&self, user: User) -> Result<Option<User>, UserRepositoryError> {
        let saved = self.source_repo.save(user.clone()).await?;
        let user_to_cache = saved.clone().unwrap_or(user);
        self.warm_up_l2_and_l1(&user_to_cache).await;
        Ok(saved)
    }

    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<User>, UserRepositoryError> {
        if let Ok(Some(user)) = self.l1_cache.find_by_id(user_id).await {
            return Ok(Some(user));
        }
        if let Ok(Some(user)) = self.l2_cache.find_by_id(user_id).await {
            self.warm_up_l1(&user).await;
            return Ok(Some(user));
        }
        let user = self.source_repo.find_by_id(user_id).await?;
        if let Some(ref user) = user {
            self.warm_up_l2_and_l1(user).await;
        }
        Ok(user)
    }

    async fn find_by_name(
        &self,
        user_name: &UserName,
    ) -> Result<Option<User>, UserRepositoryError> {
        if let Ok(Some(user)) = self.l1_cache.find_by_name(user_name).await {
            return Ok(Some(user));
        }
        if let Ok(Some(user)) = self.l2_cache.find_by_name(user_name).await {
            self.warm_up_l1(&user).await;
            return Ok(Some(user));
        }
        let user = self.source_repo.find_by_name(user_name).await?;
        if let Some(ref user) = user {
            self.warm_up_l2_and_l1(user).await;
        }
        Ok(user)
    }

    async fn find_by_email(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError> {
        if let Ok(Some(user)) = self.l1_cache.find_by_email(user_email).await {
            return Ok(Some(user));
        }
        if let Ok(Some(user)) = self.l2_cache.find_by_email(user_email).await {
            self.warm_up_l1(&user).await;
            return Ok(Some(user));
        }
        let user = self.source_repo.find_by_email(user_email).await?;
        if let Some(ref user) = user {
            self.warm_up_l2_and_l1(user).await;
        }
        Ok(user)
    }

    async fn find_by_name_or_email(
        &self,
        user_name: &UserName,
        user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError> {
        let user = self.find_by_name(user_name).await?;
        if user.is_some() {
            return Ok(user);
        }
        self.find_by_email(user_email).await
    }

    async fn update_status_as_true(
        &self,
        user_email: &UserEmail,
    ) -> Result<Option<User>, UserRepositoryError> {
        let updated = self.source_repo.update_status_as_true(user_email).await?;
        if let Some(ref user) = updated {
            self.warm_up_l2_and_l1(user).await;
        }
        Ok(updated)
    }
}
