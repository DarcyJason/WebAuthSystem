use thiserror::Error;

use crate::domain::user::errors::UserError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    UserError(#[from] UserError),
    #[error("Repository is unavailable: {0}")]
    RepoitoryUnavailable(String),
}
