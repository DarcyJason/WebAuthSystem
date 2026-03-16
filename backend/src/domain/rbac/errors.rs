use thiserror::Error;

#[derive(Debug, Error)]
pub enum PermissionCodeError {
    #[error("Invalid permission code format")]
    InvalidFormat,
}

#[derive(Debug, Error)]
pub enum PermissionIdError {
    #[error("get role id from &str error")]
    GetRoleIdFromStrError,
}

#[derive(Debug, Error)]
pub enum RoleCodeError {
    #[error("invalid role code format")]
    InvalidFormat,
}

#[derive(Debug, Error)]
pub enum RoleIdError {
    #[error("get role id from &str error")]
    GetRoleIdFromStrError,
}
