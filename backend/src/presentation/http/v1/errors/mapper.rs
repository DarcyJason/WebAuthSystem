use crate::domain::auth::errors::AuthDomainError;
use crate::domain::auth::repositories::email_verification_token_repository::EmailVerificationTokenRepositoryError;
use crate::domain::auth::services::mail_service::AuthMailServiceError;
use crate::domain::auth::services::password_service::AuthPasswordServiceError;
use crate::domain::auth::services::token_service::{
    AuthAccessTokenServiceError, AuthRefreshTokenServiceError,
};
use crate::domain::auth::value_objects::plain_password::PlainPasswordError;
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
                DomainError::AuthDomainError(auth_domain_error) => match auth_domain_error {
                    AuthDomainError::PlainPasswordError(plain_password_error) => {
                        match plain_password_error {
                            PlainPasswordError::PasswordRequired => {
                                ApiError::internal_server_error(
                                    500,
                                    PlainPasswordError::PasswordRequired.to_string(),
                                )
                            }
                            PlainPasswordError::PasswordTooShort => {
                                ApiError::internal_server_error(
                                    500,
                                    PlainPasswordError::PasswordTooShort.to_string(),
                                )
                            }
                            PlainPasswordError::PasswordTooLong => ApiError::internal_server_error(
                                500,
                                PlainPasswordError::PasswordTooLong.to_string(),
                            ),
                            PlainPasswordError::PasswordMissingDigit => {
                                ApiError::internal_server_error(
                                    500,
                                    PlainPasswordError::PasswordMissingDigit.to_string(),
                                )
                            }
                            PlainPasswordError::PasswordMissingLowerCase => {
                                ApiError::internal_server_error(
                                    500,
                                    PlainPasswordError::PasswordMissingLowerCase.to_string(),
                                )
                            }
                            PlainPasswordError::PasswordMissingUpperCase => {
                                ApiError::internal_server_error(
                                    500,
                                    PlainPasswordError::PasswordMissingUpperCase.to_string(),
                                )
                            }
                            PlainPasswordError::PasswordMissingSpetial => {
                                ApiError::internal_server_error(
                                    500,
                                    PlainPasswordError::PasswordMissingSpetial.to_string(),
                                )
                            }
                        }
                    }
                    AuthDomainError::EmailVerificationTokenRepositoryError(repo_error) => {
                        match repo_error {
                            EmailVerificationTokenRepositoryError::TokenStoreUnavailable => {
                                ApiError::internal_server_error(
                                    500,
                                    EmailVerificationTokenRepositoryError::TokenStoreUnavailable
                                        .to_string(),
                                )
                            }
                            EmailVerificationTokenRepositoryError::TokenNotFound => {
                                ApiError::internal_server_error(
                                    500,
                                    EmailVerificationTokenRepositoryError::TokenNotFound
                                        .to_string(),
                                )
                            }
                            EmailVerificationTokenRepositoryError::TokenRemoveFailed => {
                                ApiError::internal_server_error(
                                    500,
                                    EmailVerificationTokenRepositoryError::TokenRemoveFailed
                                        .to_string(),
                                )
                            }
                        }
                    }
                    AuthDomainError::AuthMailServiceError(mail_error) => match mail_error {
                        AuthMailServiceError::SendEmailFailed => ApiError::internal_server_error(
                            500,
                            AuthMailServiceError::SendEmailFailed.to_string(),
                        ),
                    },
                    AuthDomainError::AuthPasswordServiceError(password_error) => {
                        match password_error {
                            AuthPasswordServiceError::HashPasswordError => {
                                ApiError::internal_server_error(
                                    500,
                                    AuthPasswordServiceError::HashPasswordError.to_string(),
                                )
                            }
                            AuthPasswordServiceError::ParseHashedPasswordError => {
                                ApiError::internal_server_error(
                                    500,
                                    AuthPasswordServiceError::ParseHashedPasswordError.to_string(),
                                )
                            }
                        }
                    }
                    AuthDomainError::AuthAccessTokenServiceError(access_token_error) => {
                        match access_token_error {
                            AuthAccessTokenServiceError::EncodeAccessTokenFailed => {
                                ApiError::internal_server_error(
                                    500,
                                    AuthAccessTokenServiceError::EncodeAccessTokenFailed
                                        .to_string(),
                                )
                            }
                            AuthAccessTokenServiceError::DecodeAccessTokenFailed => {
                                ApiError::internal_server_error(
                                    500,
                                    AuthAccessTokenServiceError::DecodeAccessTokenFailed
                                        .to_string(),
                                )
                            }
                        }
                    }
                    AuthDomainError::AuthRefreshTokenServiceError(refresh_token_error) => {
                        match refresh_token_error {
                            AuthRefreshTokenServiceError::GenerateRefreshTokenFailed => {
                                ApiError::internal_server_error(
                                    500,
                                    AuthRefreshTokenServiceError::GenerateRefreshTokenFailed
                                        .to_string(),
                                )
                            }
                        }
                    }
                },
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
                            UserRepositoryError::PersistenceFailed => {
                                ApiError::internal_server_error(
                                    500,
                                    UserRepositoryError::PersistenceFailed.to_string(),
                                )
                            }
                            UserRepositoryError::DeserializationFailed => {
                                ApiError::internal_server_error(
                                    500,
                                    UserRepositoryError::DeserializationFailed.to_string(),
                                )
                            }
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
