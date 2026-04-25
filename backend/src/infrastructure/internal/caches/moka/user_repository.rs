use crate::infrastructure::internal::caches::moka::client::MokaClient;
use crate::infrastructure::internal::caches::user_repository::CacheUserRepository;

pub type MokaUserRepository = CacheUserRepository<MokaClient>;
