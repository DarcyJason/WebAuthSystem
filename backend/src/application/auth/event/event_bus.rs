use async_trait::async_trait;

#[async_trait]
pub trait EventBus {
    async fn publish<E: Send + Sync + 'static>(&self, event: E);
}
