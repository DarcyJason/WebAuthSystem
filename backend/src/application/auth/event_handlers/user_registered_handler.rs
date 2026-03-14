use async_trait::async_trait;

use crate::{
    application::auth::event::event_handler::EventHandler,
    domain::auth::event::user_registered::UserRegistered,
};

pub struct UserRegisteredHandler;

#[async_trait]
impl EventHandler<UserRegistered> for UserRegisteredHandler {
    async fn handle(&self, _event: &UserRegistered) {
        // println!("new user: {}", event.user_id);
    }
}
