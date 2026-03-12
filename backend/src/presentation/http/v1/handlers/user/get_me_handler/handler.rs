use crate::presentation::http::v1::errors::ApiResult;
use crate::{
    application::{
        queries::user::get_me_query::GetMeQuery, use_cases::user::get_me_case::GetMeCase,
    },
    presentation::http::v1::{
        handlers::user::get_me_handler::response::GetMeResponseData, response::ApiResponse,
        states::AppState,
    },
};
use axum::{http::HeaderMap, response::IntoResponse, Extension};
use std::sync::Arc;

pub async fn get_me_handler(
    Extension(app_state): Extension<Arc<AppState>>,
    headers: HeaderMap,
) -> ApiResult<impl IntoResponse> {
    let query = GetMeQuery::try_from(headers)?;
    let case = GetMeCase::new(
        app_state.user_repo.clone(),
        app_state.access_token_service.clone(),
    );
    let get_me_result = case.execute(query).await?;
    let response_data = GetMeResponseData::from(get_me_result);
    let response = ApiResponse::<GetMeResponseData>::ok(200, "get me successfully", response_data);
    Ok(response)
}
