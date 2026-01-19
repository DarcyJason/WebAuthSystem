use serde::Deserialize;

pub enum PlainPasswordError {
    PasswordIsRequired,
    PasswordIsTooShort,
    PasswordIsTooLong,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlainPassword(String);

impl PlainPassword {
    pub fn new(raw: String) -> Result<Self, PlainPasswordError> {
        let raw = raw.trim();
        Ok(Self(raw.to_owned()))
    }
    pub fn expose(&self) -> &str {
        &self.0
    }
}
