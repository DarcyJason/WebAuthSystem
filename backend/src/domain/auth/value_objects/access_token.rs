#[derive(Debug, Clone)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn new(raw_access_token: impl Into<String>) -> Self {
        AccessToken(raw_access_token.into())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}
