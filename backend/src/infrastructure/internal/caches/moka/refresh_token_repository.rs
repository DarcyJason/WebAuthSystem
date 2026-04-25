use crate::infrastructure::internal::caches::moka::client::MokaClient;
use crate::infrastructure::internal::caches::refresh_token_repository::CacheRefreshTokenRepository;

pub type MokaRefreshTokenRepository = CacheRefreshTokenRepository<MokaClient>;
