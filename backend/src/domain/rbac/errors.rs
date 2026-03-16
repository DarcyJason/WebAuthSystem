use thiserror::Error;

#[derive(Debug, Error)]
pub enum PermissionCodeError {
    #[error("Invalid permission code format")]
    InvalidFormat,
}

#[derive(Debug, Error)]
pub enum PermissionIdError {
    #[error("invalid permission id format")]
    InvalidFormat,
}

#[derive(Debug, Error)]
pub enum RoleCodeError {
    #[error("invalid role code format")]
    InvalidFormat,
}

#[derive(Debug, Error)]
pub enum RoleIdError {
    #[error("invalid role id format")]
    InvalidFormat,
}
