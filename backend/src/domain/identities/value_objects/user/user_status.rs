use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "user_status", rename_all = "PascalCase")]
pub enum UserStatus {
    EmailNotVerified,
    Active,
    Banned,
}

impl UserStatus {
    pub fn new() -> Self {
        UserStatus::EmailNotVerified
    }
    pub fn value(&self) -> Self {
        self.to_owned()
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            UserStatus::EmailNotVerified => "EmailNotVerified",
            UserStatus::Active => "Active",
            UserStatus::Banned => "Banned",
        }
    }
    pub fn update_status(&mut self, user_status: &UserStatus) {
        *self = user_status.to_owned();
    }
}
