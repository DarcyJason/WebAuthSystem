pub enum AuthError {
    InvalidCredentials,
    UserAlreadyExists,
    UserNotFound,
    GenerateAccessTokenFailed,
    GenerateRefreshTokenFailed,
    VerifyAccessTokenFailed,
}
