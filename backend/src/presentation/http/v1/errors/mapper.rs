use crate::domain::errors::DomainError;
use crate::domain::user::errors::UserDomainError;
use crate::domain::user::repositories::user_repository::UserRepositoryError;
use crate::domain::user::value_objects::user_email::UserEmailError;
use crate::domain::user::value_objects::user_id::UserIdError;
use crate::{application::errors::AppError, presentation::http::v1::errors::api_error::ApiError};

impl From<AppError> for ApiError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::DomainError(domain_error) => match domain_error {
                DomainError::UserDomainError(user_domain_error) => match user_domain_error {
                    UserDomainError::UserIdError(user_id_error) => match user_id_error {
                        UserIdError::GetUserIdFromStrFailed => ApiError::internal_server_error(
                            500,
                            UserIdError::GetUserIdFromStrFailed.to_string(),
                        ),
                    },
                    UserDomainError::UserEmailError(user_email_error) => match user_email_error {
                        UserEmailError::UserEmailRequired => ApiError::internal_server_error(
                            500,
                            UserEmailError::UserEmailRequired.to_string(),
                        ),
                        UserEmailError::UserEmailInvalid => ApiError::internal_server_error(
                            500,
                            UserEmailError::UserEmailInvalid.to_string(),
                        ),
                    },
                    UserDomainError::UserRepositoryError(user_repository_error) => {
                        match user_repository_error {
                            UserRepositoryError::StorageUnavailable => {
                                ApiError::internal_server_error(
                                    500,
                                    UserRepositoryError::StorageUnavailable.to_string(),
                                )
                            }
                            UserRepositoryError::PersistFailed => ApiError::internal_server_error(
                                500,
                                UserRepositoryError::PersistFailed.to_string(),
                            ),
                            UserRepositoryError::DataCorrupted => ApiError::internal_server_error(
                                500,
                                UserRepositoryError::DataCorrupted.to_string(),
                            ),
                        }
                    }
                },
            },
            AppError::StorageError => ApiError::internal_server_error(500, "Storage error"),
            AppError::UserAlreadyExists => ApiError::conflict("User already exists"),
            AppError::CreateUserFailed => {
                ApiError::internal_server_error(500, "Failed to create user")
            }
            AppError::UserNotFound => ApiError::not_found("User not found"),
            AppError::HashPasswordFailed => {
                ApiError::internal_server_error(500, "Failed to hash password")
            }
            AppError::ParseHashedPasswordFailed => {
                ApiError::internal_server_error(500, "Failed to parse hashed password")
            }
            AppError::CredentialsInvalid => ApiError::unauthorized("Invalid credentials"),
            AppError::EmailNotVerified => ApiError::unauthorized("Email not verified"),
            AppError::EncodeAccessTokenFailed => {
                ApiError::internal_server_error(500, "Failed to encode access_token")
            }
            AppError::DecodeAccessTokenFailed => {
                ApiError::internal_server_error(500, "Failed to decode access_token")
            }
            AppError::GenerateRefreshTokenFailed => {
                ApiError::internal_server_error(500, "Failed to generate refresh_token")
            }
            AppError::SystemOwnerEmailInvalid => {
                ApiError::internal_server_error(500, "System owner email invalid")
            }
            AppError::SendVerificationEmailFailed => {
                ApiError::internal_server_error(500, "Failed to send email")
            }
            AppError::SaveEmailVerificationTokenFailed => {
                ApiError::internal_server_error(500, "Failed to save email verification token")
            }
            AppError::GetEmailVerificationTokenFailed => {
                ApiError::internal_server_error(500, "Failed to get email verification token")
            }
            AppError::EmailVerificationTokenNotFound => {
                ApiError::not_found("Email verification token not found")
            }
            AppError::EmailVerificationTokenInvalid => {
                ApiError::internal_server_error(500, "Email verification token is invalid")
            }
        }
    }
}
