use std::fmt::Display;

#[derive(Debug)]
pub enum CacheOperation {
    Get,
    Set,
    BatchGet,
    BatchSet,
    Delete,
    Serialize,
    Deserialize,
}

impl Display for CacheOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheOperation::Get => write!(f, "get"),
            CacheOperation::Set => write!(f, "set"),
            CacheOperation::BatchGet => write!(f, "batch_get"),
            CacheOperation::BatchSet => write!(f, "batch_set"),
            CacheOperation::Delete => write!(f, "delete"),
            CacheOperation::Serialize => write!(f, "serialize"),
            CacheOperation::Deserialize => write!(f, "deserialize"),
        }
    }
}
