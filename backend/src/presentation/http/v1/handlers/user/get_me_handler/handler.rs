use axum::{extract::State, http::HeaderMap, response::IntoResponse};

use crate::{
    app_state::AppState,
    application::{
        queries::user::get_me_query::GetMeQuery, use_cases::user::get_me_case::GetMeCase,
    },
    presentation::http::v1::{
        errors::api_error::ApiResult, handlers::user::get_me_handler::response::GetMeResponseData,
        response::ApiResponse,
    },
};

pub async fn get_me_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> ApiResult<impl IntoResponse> {
    let query = GetMeQuery::try_from(headers)?;
    let case = GetMeCase::new(
        app_state.user_repo.clone(),
        app_state.auth_access_token_service.clone(),
    );
    let get_me_result = case.execute(query).await?;
    let response_data = GetMeResponseData::from(get_me_result);
    let response = ApiResponse::<GetMeResponseData>::ok(200, "get me successfully", response_data);
    Ok(response)
}
