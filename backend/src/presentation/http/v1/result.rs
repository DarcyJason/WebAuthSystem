use crate::application::errors::ApplicationError;

pub type AppResult<T> = Result<T, ApplicationError>;