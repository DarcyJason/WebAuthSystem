use crate::application::app_state::AppState;
use crate::domain::auth::services::access_token_service::AccessTokenService;
use crate::domain::auth::value_objects::tokens::access_token::AccessToken;
use crate::domain::user::aggregates::user::UserEntity;
use crate::domain::user::repositories::user_repository::UserQueryRepository;
use crate::domain::user::value_objects::user::user_id::UserId;
use crate::presentation::http::v1::error::{ApiError, ApiResult};
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthMiddleware {
    pub user: UserEntity,
}

pub async fn auth(
    State(app_state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> ApiResult<impl IntoResponse> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or(ApiError::Unauthorized {
            message: "missing or invalid authorization header".to_string(),
        })?;
    let access_token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(ApiError::Unauthorized {
            message: "invalid bearer token format".to_string(),
        })?;
    let claims = app_state
        .access_token_service
        .decode(&AccessToken::new(access_token.to_string()))
        .map_err(|_| ApiError::Unauthorized {
            message: "invalid or expired access token".to_string(),
        })?;
    let user_id =
        UserId::from_raw_user_id(claims.sub().to_string()).map_err(|_| ApiError::Unauthorized {
            message: "invalid or expired access token".to_string(),
        })?;
    let user = app_state
        .user_repo
        .get_by_id(&user_id)
        .await
        .map_err(|_| ApiError::InternalServerError {
            code: 500,
            message: "internal server error".to_string(),
        })?;
    let user = user.ok_or(ApiError::NotFound {
        message: "user not found".to_string(),
    })?;
    if claims.ver() != user.access_token_version().value().to_string() {
        return Err(ApiError::Unauthorized {
            message: "invalid or expired access token".to_string(),
        });
    }
    req.extensions_mut().insert(AuthMiddleware { user });
    Ok(next.run(req).await)
}
