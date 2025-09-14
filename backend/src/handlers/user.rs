use ntex::web::{HttpRequest, Responder};

use crate::{
    dtos::{api_response::ApiResponse, response::user::UserResponse},
    errors::app_error::{AppError, AppResult},
    models::user::User,
};

pub async fn me_handler(req: HttpRequest) -> AppResult<impl Responder> {
    let extensions = req.extensions();
    let user = extensions
        .get::<User>()
        .ok_or(AppError::InternalServerError(
            "User not found in request extensions".to_string(),
        ))?;
    let user_response = UserResponse::from(user.clone());
    Ok(ApiResponse::success("User profile fetched", user_response))
}
