pub struct LayeredRefreshTokenRepository;

impl LayeredRefreshTokenRepository {
    pub fn new() -> Self {
        LayeredRefreshTokenRepository
    }
}

impl Default for LayeredRefreshTokenRepository {
    fn default() -> Self {
        Self::new()
    }
}
