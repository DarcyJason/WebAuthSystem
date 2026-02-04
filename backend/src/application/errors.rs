pub type AppResult<T> = Result<T, AppError>;

pub enum AppError {
    SurrealDBError,
    UserAlreadyExists,
    CreateUserFailed,
    UserNotFound,
    HashPasswordFailed,
    ParseHashedPasswordFailed,
    WrongIncredentials,
    EmailNotVerified,
    EncodeAccessTokenFailed,
    DecodeAccessTokenFailed,
    GenerateRefreshTokenFailed,
}
