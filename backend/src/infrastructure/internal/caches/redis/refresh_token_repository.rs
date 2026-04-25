use crate::infrastructure::internal::caches::redis::client::RedisClient;
use crate::infrastructure::internal::caches::refresh_token_repository::CacheRefreshTokenRepository;

pub type RedisRefreshTokenRepository = CacheRefreshTokenRepository<RedisClient>;
