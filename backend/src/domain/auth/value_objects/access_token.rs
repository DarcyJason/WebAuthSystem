#[derive(Debug, Clone)]
pub struct AccessToken(pub String);

impl AccessToken {
    pub fn new(raw_access_token: String) -> Self {
        AccessToken(raw_access_token)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
