use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use super::event_bus::EventBus;
use super::event_handler::EventHandler;

#[async_trait]
// 类型擦除的处理器，让不同的 EventHandler<E> 能存进同一个 map。
trait ErasedEventHandler: Send + Sync {
    async fn handle_boxed(&self, event: &(dyn Any + Send + Sync));
}

// 包装具体的 EventHandler<E>，并以擦除后的接口暴露。
struct DynHandler<E> {
    inner: Arc<dyn EventHandler<E>>,
}

#[async_trait]
impl<E: Send + Sync + 'static> ErasedEventHandler for DynHandler<E> {
    async fn handle_boxed(&self, event: &(dyn Any + Send + Sync)) {
        if let Some(event) = event.downcast_ref::<E>() {
            self.inner.handle(event).await;
        }
    }
}

pub struct SimpleEventBus {
    // 事件类型 -> 关心该事件的处理器列表。
    handlers: HashMap<TypeId, Vec<Arc<dyn ErasedEventHandler>>>,
}

impl SimpleEventBus {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    // 为具体事件类型 E 注册处理器。
    pub fn register<E: Send + Sync + 'static>(&mut self, handler: Arc<dyn EventHandler<E>>) {
        self.handlers
            .entry(TypeId::of::<E>())
            .or_default()
            .push(Arc::new(DynHandler { inner: handler }));
    }
}

impl Default for SimpleEventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventBus for SimpleEventBus {
    async fn publish<E: Send + Sync + 'static>(&self, event: E) {
        // 按具体事件类型查找并分发给对应处理器。
        if let Some(handlers) = self.handlers.get(&TypeId::of::<E>()) {
            for handler in handlers {
                handler.handle_boxed(&event).await;
            }
        }
    }
}
