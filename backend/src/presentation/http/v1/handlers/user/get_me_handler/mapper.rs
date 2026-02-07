use axum::http::HeaderMap;
use thiserror::Error;

use crate::{
    application::{
        queries::user::get_me_query::GetMeQuery,
        results::queries_results::user::get_me_result::GetMeResult,
    },
    domain::auth::value_objects::access_token::AccessToken,
    presentation::http::v1::{
        errors::api_error::ApiError, handlers::user::get_me_handler::response::GetMeResponseData,
    },
};

#[derive(Debug, Error)]
pub enum GetMeHeaderError {
    #[error("Invalid access_token")]
    InvalidAccessToken,
}

impl TryFrom<HeaderMap> for GetMeQuery {
    type Error = GetMeHeaderError;
    fn try_from(headers: HeaderMap) -> Result<Self, Self::Error> {
        let authorization_value = headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(GetMeHeaderError::InvalidAccessToken)?;
        let raw_access_token = match authorization_value.strip_prefix("Bearer ") {
            Some(raw_token) => raw_token,
            None => return Err(GetMeHeaderError::InvalidAccessToken),
        };
        let access_token = AccessToken::new(raw_access_token);
        Ok(GetMeQuery { access_token })
    }
}

impl From<GetMeHeaderError> for ApiError {
    fn from(err: GetMeHeaderError) -> Self {
        ApiError::Unauthorized {
            message: err.to_string(),
        }
    }
}

impl From<GetMeResult> for GetMeResponseData {
    fn from(result: GetMeResult) -> Self {
        GetMeResponseData {
            name: result.user_name.value().to_owned(),
            email: result.user_name.value().to_owned(),
            created_at: result.created_at.value().to_string().to_owned(),
        }
    }
}
