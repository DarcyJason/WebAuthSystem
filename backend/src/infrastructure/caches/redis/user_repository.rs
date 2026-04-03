use crate::infrastructure::caches::redis::client::RedisClient;
use crate::infrastructure::caches::user_repository::CacheUserRepository;

pub type RedisUserRepository = CacheUserRepository<RedisClient>;
