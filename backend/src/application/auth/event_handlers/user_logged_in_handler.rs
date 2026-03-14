use crate::application::auth::event::event_handler::EventHandler;
use crate::domain::auth::event::user_logged_in::UserLoggedIn;
use async_trait::async_trait;

pub struct UserLoggedInHandler;

#[async_trait]
impl EventHandler<UserLoggedIn> for UserLoggedInHandler {
    async fn handle(&self, _event: &UserLoggedIn) {
        // println!("user logged in: {}", event.user_id);
    }
}
