#[derive(Debug, Clone)]
pub struct RefreshToken(String);

impl RefreshToken {
    pub fn new(refresh_token_value: impl Into<String>) -> Self {
        Self(refresh_token_value.into())
    }
    pub fn value(&self) -> String {
        self.0.to_owned()
    }
}
