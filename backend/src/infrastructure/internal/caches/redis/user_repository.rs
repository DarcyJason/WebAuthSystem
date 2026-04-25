use crate::infrastructure::internal::caches::redis::client::RedisClient;
use crate::infrastructure::internal::caches::user_repository::CacheUserRepository;

pub type RedisUserRepository = CacheUserRepository<RedisClient>;
