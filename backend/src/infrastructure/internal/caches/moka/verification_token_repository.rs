use crate::infrastructure::internal::caches::moka::client::MokaClient;
use crate::infrastructure::internal::caches::verification_token_repository::CacheVerificationTokenRepository;

pub type MokaVerificationTokenRepository = CacheVerificationTokenRepository<MokaClient>;
