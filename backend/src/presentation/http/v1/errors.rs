use crate::application::errors::{ApplicationError, ApplicationResult};
use crate::domain::auth::errors::AuthError;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserError;
use crate::presentation::http::v1::response::ApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

impl UserError {
    fn error_info(&self) -> (StatusCode, u16, String) {
        match self {
            UserError::UsernameIsrequired
            | UserError::UsernameIsInvalid
            | UserError::UsernameIsTooShort
            | UserError::UsernameIsTooLong => (StatusCode::BAD_REQUEST, 10001, self.to_string()),
            UserError::EmailIsRequired | UserError::EmailIsInvalid => {
                (StatusCode::BAD_REQUEST, 10002, self.to_string())
            }
            UserError::PasswordIsRequired
            | UserError::PasswordIsInvalid
            | UserError::PasswordIsTooshort
            | UserError::PasswordIsTooLong => (StatusCode::BAD_REQUEST, 10003, self.to_string()),
            UserError::PasswordsNotMatch => (StatusCode::BAD_REQUEST, 10004, self.to_string()),
            UserError::ConfirmPasswordIsRequired
            | UserError::ConfirmPasswordIsInvalid
            | UserError::ConfirmPasswordIsTooshort
            | UserError::ConfirmPasswordIsTooLong => {
                (StatusCode::BAD_REQUEST, 10005, self.to_string())
            }
            UserError::HashPasswordError | UserError::ParseHashPasswordError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                10001,
                "Internal server error".to_string(),
            ),
        }
    }
}

impl AuthError {
    fn error_info(&self) -> (StatusCode, u16, String) {
        match self {
            AuthError::PasswordTooShort | AuthError::PasswordTooLong => {
                (StatusCode::BAD_REQUEST, 20001, self.to_string())
            }
            AuthError::InvalidCredentials => (StatusCode::BAD_REQUEST, 20002, self.to_string()),
        }
    }
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        let (http_status, business_code, message) = match self {
            ApplicationError::DomainError(DomainError::UserError(ref user_error)) => {
                user_error.error_info()
            }
            ApplicationError::DomainError(DomainError::AuthError(ref auth_error)) => {
                auth_error.error_info()
            }
            ApplicationError::DomainError(DomainError::NotFound(_)) => {
                (StatusCode::NOT_FOUND, 30400, self.to_string())
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

pub type ApiResult<T> = ApplicationResult<T>;
