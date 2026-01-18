use crate::application::errors::ApplicationError;
use crate::domain::error::DomainError;
use crate::presentation::http::v1::response::ApiResponse;
use axum::response::IntoResponse;

pub type ApiResult<T> = Result<T, ApplicationError>;

impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::response::Response {
        let (code, message) = match self {
            ApplicationError::DomainError(domain_err) => match domain_err {
                DomainError::NotFound(msg) => (404, msg),
                DomainError::Conflict(msg) => (409, msg),
                DomainError::Validation(msg) => (400, msg),
                DomainError::Unauthorized => (401, "Unauthorized".to_string()),
                DomainError::Forbidden => (403, "Forbidden".to_string()),
                DomainError::Invariant(msg) => (400, msg),
                DomainError::Repository(msg) => (500, msg),
            },
            ApplicationError::Infrastructure => (500, "Infrastructure error".to_string()),
            ApplicationError::Unauthorized => (401, "Unauthorized".to_string()),
            ApplicationError::Unexpected => (500, "Unexpected error".to_string()),
        };

        ApiResponse::<()>::err(code, message).into_response()
    }
}
