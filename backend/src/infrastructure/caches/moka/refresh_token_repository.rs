pub struct MokaRefreshTokenRepository;

impl MokaRefreshTokenRepository {
    pub fn new() -> Self {
        MokaRefreshTokenRepository
    }
}

impl Default for MokaRefreshTokenRepository {
    fn default() -> Self {
        Self::new()
    }
}
