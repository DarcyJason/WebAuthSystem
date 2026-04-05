#[derive(Debug, Clone)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn new(access_token_value: impl Into<String>) -> Self {
        Self(access_token_value.into())
    }
    pub fn value(&self) -> String {
        self.0.to_owned()
    }
}
