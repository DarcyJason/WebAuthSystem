use std::fmt::Display;

#[derive(Debug)]
pub enum CacheLayer {
    L1Moka,
    L2Redis,
}

impl Display for CacheLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheLayer::L1Moka => write!(f, "L1Moka"),
            CacheLayer::L2Redis => write!(f, "L2Redis"),
        }
    }
}
