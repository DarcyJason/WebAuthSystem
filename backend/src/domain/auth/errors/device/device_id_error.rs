use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceIdError {
    #[error("invalid device id format")]
    InvalidFormat,
}
