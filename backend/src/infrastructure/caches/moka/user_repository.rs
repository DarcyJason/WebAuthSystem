use crate::infrastructure::caches::moka::client::MokaClient;
use crate::infrastructure::caches::user_repository::CacheUserRepository;

pub type MokaUserRepository = CacheUserRepository<MokaClient>;
