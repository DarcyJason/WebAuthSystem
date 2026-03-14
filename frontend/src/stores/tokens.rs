pub struct TokenState {
    pub access_token: String,
}

impl TokenState {
    pub fn new() -> Self {
        TokenState {
            access_token: String::new(),
        }
    }
}
