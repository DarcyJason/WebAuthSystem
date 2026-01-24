use crate::application::errors::{ApplicationError, ApplicationResult};
use crate::application::queries::user::get_me::GetMeResult;
use crate::domain::auth::repositories::token::AuthTokenRepository;
use crate::domain::auth::value_objects::access_token::AccessToken;
use crate::domain::user::repositories::db::UserRepository;

pub struct GetMeCase<SU, T>
where
    SU: UserRepository,
    T: AuthTokenRepository,
{
    user_repo: SU,
    token_repo: T,
}

impl<SU, T> GetMeCase<SU, T>
where
    SU: UserRepository,
    T: AuthTokenRepository,
{
    pub fn new(user_repo: SU, token_repo: T) -> Self {
        GetMeCase {
            user_repo,
            token_repo,
        }
    }
    pub async fn execute(&self, access_token: AccessToken) -> ApplicationResult<GetMeResult> {
        let access_claims = self
            .token_repo
            .decode_access_token(access_token.as_str())
            .map_err(|_| ApplicationError::AccessTokenInvalid)?;
        let user = self
            .user_repo
            .find_by_id(&access_claims.sub)
            .await
            .map_err(|_| ApplicationError::UserNotFound)?
            .ok_or(ApplicationError::UserNotFound)?;
        Ok(GetMeResult::from(user))
    }
}
