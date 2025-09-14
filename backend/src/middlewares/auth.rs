use ntex::{
    ServiceCtx,
    http::HttpMessage,
    service::{Middleware, Service},
    web,
};

use crate::{
    errors::app_error::AppError, models::user::User, repositories::auth::AuthRepository,
    state::AppState, utils::token::validate_access_token,
};

pub struct Auth;

impl<S> Middleware<S> for Auth {
    type Service = AuthMiddleware<S>;
    fn create(&self, service: S) -> Self::Service {
        AuthMiddleware { service }
    }
}
pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for AuthMiddleware<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_ready!(service);

    async fn call(
        &self,
        req: web::WebRequest<Err>,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> {
        let app_state = req
            .app_state::<AppState>()
            .expect("AppState is not set")
            .clone();
        let access_token = match req.cookie("access_token") {
            Some(cookie) => cookie.value().to_string(),
            None => return Err(web::Error::from(AppError::AuthenticationError)),
        };
        let user_id = match validate_access_token(
            access_token,
            app_state.config.token.jwt_secret_key.as_bytes(),
        ) {
            Ok(id) => id,
            Err(_) => return Err(web::Error::from(AppError::InvalidToken)),
        };
        let user = match app_state.db_client.find_user_by_id(user_id).await {
            Ok(Some(user)) => user,
            _ => return Err(web::Error::from(AppError::UserNotFound)),
        };
        req.extensions_mut().insert::<User>(user);
        let res = ctx.call(&self.service, req).await?;
        Ok(res)
    }
}
