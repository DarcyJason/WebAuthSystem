use crate::domain::auth::event::user_logged_in::UserLoggedIn;

pub mod user_logged_in;
pub mod user_registered;

pub enum AuthEvent {
    UserLoggedIn(UserLoggedIn),
}
