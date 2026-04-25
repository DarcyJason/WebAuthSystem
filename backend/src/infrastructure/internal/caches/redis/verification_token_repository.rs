use crate::infrastructure::internal::caches::redis::client::RedisClient;
use crate::infrastructure::internal::caches::verification_token_repository::CacheVerificationTokenRepository;

pub type RedisVerificationTokenRepository = CacheVerificationTokenRepository<RedisClient>;
