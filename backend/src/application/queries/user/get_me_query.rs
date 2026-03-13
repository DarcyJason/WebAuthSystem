use crate::domain::auth::value_objects::tokens::access_token::AccessToken;

pub struct GetMeQuery {
    pub access_token: AccessToken,
}
