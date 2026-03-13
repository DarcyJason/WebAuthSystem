#[derive(Debug, Clone)]
pub struct RefreshToken(String);

impl RefreshToken {
    pub fn new(raw_refresh_token: impl Into<String>) -> Self {
        RefreshToken(raw_refresh_token.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
