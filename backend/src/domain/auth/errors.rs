pub enum AuthError {
    InvalidCredentials,
    UserAlreadyExists,
    EncodeJWTError,
    DecodeJWTError,
}
