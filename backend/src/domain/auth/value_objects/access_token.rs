#[derive(Debug, Clone)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn new(raw: String) -> Self {
        AccessToken(raw)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
