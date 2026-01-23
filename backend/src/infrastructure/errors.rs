use crate::infrastructure::cache::redis::errors::RedisError;
use crate::infrastructure::persistence::surreal::errors::SurrealDBError;
use crate::infrastructure::token::errors::TokenError;

pub type InfraResult<T> = Result<T, InfrastructureError>;

pub enum InfrastructureError {
    RedisError(RedisError),
    SurrealDBError(SurrealDBError),
    TokenError(TokenError),
}

impl From<RedisError> for InfrastructureError {
    fn from(err: RedisError) -> Self {
        InfrastructureError::RedisError(err)
    }
}

impl From<SurrealDBError> for InfrastructureError {
    fn from(err: SurrealDBError) -> Self {
        InfrastructureError::SurrealDBError(err)
    }
}

impl From<TokenError> for InfrastructureError {
    fn from(err: TokenError) -> Self {
        InfrastructureError::TokenError(err)
    }
}
