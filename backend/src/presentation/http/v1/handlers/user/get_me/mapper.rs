use crate::application::queries::user::get_me::GetMeResult;
use crate::presentation::http::v1::handlers::user::get_me::response::GetMeResponseData;

impl From<GetMeResult> for GetMeResponseData {
    fn from(result: GetMeResult) -> Self {
        GetMeResponseData {
            user_id: result.user_id.to_string(),
            username: result.username.to_string(),
            email: result.email.to_string(),
        }
    }
}
