use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceIdError {
    #[error("get device id from &str error")]
    GetDeviceIdFromStrError,
}
