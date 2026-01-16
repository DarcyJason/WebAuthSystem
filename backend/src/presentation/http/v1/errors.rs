use crate::application::errors::ApplicationError;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserError;
use crate::presentation::http::v1::response::ApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        let (http_status, business_code, message) = match self {
            ApplicationError::UserError(UserError::UsernameIsrequired) => {
                (StatusCode::BAD_REQUEST, 40001, self.to_string())
            }
            ApplicationError::UserError(UserError::UsernameIsTooShort) => {
                (StatusCode::BAD_REQUEST, 40001, self.to_string())
            }
            ApplicationError::UserError(UserError::UsernameIsTooLong) => {
                (StatusCode::BAD_REQUEST, 40001, self.to_string())
            }
            ApplicationError::UserError(UserError::EmailIsRequired) => {
                (StatusCode::BAD_REQUEST, 40002, self.to_string())
            }
            ApplicationError::UserError(UserError::EmailIsInvalid) => {
                (StatusCode::BAD_REQUEST, 40002, self.to_string())
            }
            ApplicationError::UserError(UserError::PasswordIsRequired) => {
                (StatusCode::BAD_REQUEST, 40003, self.to_string())
            }
            ApplicationError::UserError(UserError::PasswordIsInvalid) => {
                (StatusCode::BAD_REQUEST, 40003, self.to_string())
            }
            ApplicationError::UserError(UserError::HashPasswordError) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                50001,
                "Internal server error".to_string(),
            ),
            ApplicationError::UserError(UserError::ParseHashPasswordError) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                50001,
                "Internal server error".to_string(),
            ),
            ApplicationError::UserError(UserError::PasswordsNotMatch) => {
                (StatusCode::BAD_REQUEST, 40004, self.to_string())
            }
            ApplicationError::DomainError(DomainError::NotFound(_)) => {
                (StatusCode::NOT_FOUND, 40400, self.to_string())
            }
            ApplicationError::DomainError(DomainError::Duplicate(_)) => {
                (StatusCode::CONFLICT, 40900, self.to_string())
            }
            ApplicationError::DomainError(DomainError::Validation(_)) => {
                (StatusCode::BAD_REQUEST, 40000, self.to_string())
            }
            ApplicationError::DomainError(DomainError::Repository(_)) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                50001,
                "Internal server error".to_string(),
            ),
            ApplicationError::HealthError(_) => {
                (StatusCode::SERVICE_UNAVAILABLE, 50300, self.to_string())
            }
            ApplicationError::ValidationFailed(_) => {
                (StatusCode::BAD_REQUEST, 40000, self.to_string())
            }
            ApplicationError::Unauthorized(_) => {
                (StatusCode::UNAUTHORIZED, 40100, self.to_string())
            }
            ApplicationError::NotFound(_) => (StatusCode::NOT_FOUND, 40400, self.to_string()),
            ApplicationError::Conflict(_) => (StatusCode::CONFLICT, 40900, self.to_string()),
        };

        let response = ApiResponse::<()>::err(business_code, &message);
        (http_status, axum::Json(response)).into_response()
    }
}

pub type AppResult<T> = Result<T, ApplicationError>;
