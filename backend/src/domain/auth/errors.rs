use std::fmt::Display;

pub enum AuthError {
    InvalidCredentials,
    UserAlreadyExists,
    UserNotFound,
    GenerateAccessTokenFailed,
    GenerateRefreshTokenFailed,
    VerifyAccessTokenFailed,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthError::UserAlreadyExists => write!(f, "User already exists"),
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::GenerateAccessTokenFailed => write!(f, "Failed to generate access token"),
            AuthError::GenerateRefreshTokenFailed => write!(f, "Failed to generate refresh token"),
            AuthError::VerifyAccessTokenFailed => write!(f, "Failed to verify access token"),
        }
    }
}
