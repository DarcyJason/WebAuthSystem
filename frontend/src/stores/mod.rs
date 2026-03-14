use crate::stores::tokens::TokenState;

pub mod tokens;

pub struct AppState {
    pub token: TokenState,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            token: TokenState::new(),
        }
    }
}
