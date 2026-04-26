use crate::presentation::http::v1::handlers::change_password_handler::request::ChangePasswordRequestPayload;
use crate::presentation::http::v1::handlers::change_password_handler::response::ChangePasswordResponseData;
use crate::presentation::http::v1::handlers::forgot_password_handler::request::ForgotPasswordRequestPayload;
use crate::presentation::http::v1::handlers::forgot_password_handler::response::ForgotPasswordResponseData;
use crate::presentation::http::v1::handlers::get_me_handler::response::GetMeResponseData;
use crate::presentation::http::v1::handlers::login_handler::request::LoginRequestPayload;
use crate::presentation::http::v1::handlers::login_handler::response::LoginResponseData;
use crate::presentation::http::v1::handlers::logout_handler::response::LogoutResponseData;
use crate::presentation::http::v1::handlers::register_handler::request::RegisterRequestPayload;
use crate::presentation::http::v1::handlers::register_handler::response::RegisterResponseData;
use crate::presentation::http::v1::handlers::resend_verification_handler::request::ResendVerificationRequestPayload;
use crate::presentation::http::v1::handlers::resend_verification_handler::response::ResendVerificationResponseData;
use crate::presentation::http::v1::handlers::reset_password_handler::request::ResetPasswordRequestPayload;
use crate::presentation::http::v1::handlers::reset_password_handler::response::ResetPasswordResponseData;
use crate::presentation::http::v1::handlers::rotate_refresh_token_handler::response::RotateRefreshTokenResponseData;
use crate::presentation::http::v1::handlers::verify_handler::request::VerifyRequestPayload;
use crate::presentation::http::v1::handlers::verify_handler::response::VerifyResponseData;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

struct BearerAuth;

impl Modify for BearerAuth {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    info(title = "WebAuthSystem API", version = "1.0.0"),
    modifiers(&BearerAuth),
    paths(
        crate::presentation::http::v1::handlers::register_handler::register_handler,
        crate::presentation::http::v1::handlers::login_handler::login_handler,
        crate::presentation::http::v1::handlers::logout_handler::logout_handler,
        crate::presentation::http::v1::handlers::verify_handler::verify_handler,
        crate::presentation::http::v1::handlers::resend_verification_handler::resend_verification_handler,
        crate::presentation::http::v1::handlers::forgot_password_handler::forgot_password_handler,
        crate::presentation::http::v1::handlers::reset_password_handler::reset_password_handler,
        crate::presentation::http::v1::handlers::rotate_refresh_token_handler::rotate_refresh_token_handler,
        crate::presentation::http::v1::handlers::get_me_handler::get_me_handler,
        crate::presentation::http::v1::handlers::change_password_handler::change_password_handler,
    ),
    components(schemas(
        RegisterRequestPayload,
        RegisterResponseData,
        LoginRequestPayload,
        LoginResponseData,
        LogoutResponseData,
        VerifyRequestPayload,
        VerifyResponseData,
        ResendVerificationRequestPayload,
        ResendVerificationResponseData,
        ForgotPasswordRequestPayload,
        ForgotPasswordResponseData,
        ResetPasswordRequestPayload,
        ResetPasswordResponseData,
        RotateRefreshTokenResponseData,
        GetMeResponseData,
        ChangePasswordRequestPayload,
        ChangePasswordResponseData,
    )),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "User", description = "User management endpoints"),
    )
)]
pub struct ApiDoc;
