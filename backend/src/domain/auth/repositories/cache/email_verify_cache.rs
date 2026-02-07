use async_trait::async_trait;

#[async_trait]
pub trait EmailVerifyCache: Send + Sync {
    async fn save_email(&self);
    async fn get_email(&self);
    async fn delete_email(&self);
}
