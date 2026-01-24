use std::fmt::Display;

pub enum UserError {
    CreateUserFailed,
    UserNotFound,
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::CreateUserFailed => write!(f, "Failed to create user"),
            UserError::UserNotFound => write!(f, "User not found"),
        }
    }
}
