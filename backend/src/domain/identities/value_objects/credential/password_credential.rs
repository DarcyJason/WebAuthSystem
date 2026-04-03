use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct PasswordCredential(String);

impl PasswordCredential {
    pub fn new(password_hash: impl Into<String>) -> PasswordCredential {
        PasswordCredential(password_hash.into())
    }
    pub fn value(&self) -> String {
        self.0.to_owned()
    }
    pub fn update_password_credential(&mut self, new_password_hash: &PasswordCredential) {
        *self = new_password_hash.to_owned();
    }
}
