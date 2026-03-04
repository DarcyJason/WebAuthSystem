pub struct RedisRefreshTokenRepository;

impl RedisRefreshTokenRepository {
    pub fn new() -> Self {
        RedisRefreshTokenRepository
    }
}

impl Default for RedisRefreshTokenRepository {
    fn default() -> Self {
        Self::new()
    }
}
