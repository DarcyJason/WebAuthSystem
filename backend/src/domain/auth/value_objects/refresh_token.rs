#[derive(Debug, Clone)]
pub struct RefreshToken(String);

impl RefreshToken {
    pub fn new(refresh_token: String) -> Self {
        RefreshToken(refresh_token)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
