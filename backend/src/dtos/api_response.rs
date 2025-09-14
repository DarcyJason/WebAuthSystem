use cookie::Cookie;
use ntex::{
    http::StatusCode,
    web::{self, HttpResponse, Responder},
};
use serde::Serialize;
use std::future::ready;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub status_code: u16,
    pub message: String,
    pub data: Option<T>,
    #[serde(skip_serializing)]
    pub cookies: Vec<Cookie<'static>>,
}

impl<T> ApiResponse<T> {
    pub fn success(msg: &str, data: T) -> Self {
        Self {
            status: "success".to_string(),
            status_code: StatusCode::OK.as_u16(),
            message: msg.to_string(),
            data: Some(data),
            cookies: Vec::new(),
        }
    }

    pub fn error(status: StatusCode, msg: &str) -> Self {
        Self {
            status: "error".to_string(),
            status_code: status.as_u16(),
            message: msg.to_string(),
            data: None,
            cookies: Vec::new(),
        }
    }
    pub fn add_cookie(mut self, cookie: Cookie<'static>) -> Self {
        self.cookies.push(cookie);
        self
    }
    pub fn with_tokens(self, access_token: &str, refresh_token: Option<&str>) -> Self {
        let mut cookies = vec![];
        let mut access_cookie = Cookie::new("access_token", access_token.to_owned());
        access_cookie.set_path("/");
        access_cookie.set_secure(true);
        access_cookie.set_http_only(true);
        access_cookie.set_max_age(cookie::time::Duration::hours(1));
        cookies.push(access_cookie);
        if let Some(refresh_token) = refresh_token {
            let mut refresh_cookie = Cookie::new("refresh_token", refresh_token.to_owned());
            refresh_cookie.set_path("/");
            refresh_cookie.set_secure(true);
            refresh_cookie.set_http_only(true);
            refresh_cookie.set_same_site(cookie::SameSite::Strict);
            refresh_cookie.set_max_age(cookie::time::Duration::days(7));
            cookies.push(refresh_cookie);
        }
        Self { cookies, ..self }
    }
    pub fn revoke_tokens(self) -> Self {
        let mut access_cookie = Cookie::new("access_token", "");
        access_cookie.set_path("/");
        access_cookie.set_max_age(cookie::time::Duration::ZERO);
        let mut refresh_cookie = Cookie::new("refresh_token", "");
        refresh_cookie.set_path("/");
        refresh_cookie.set_max_age(cookie::time::Duration::ZERO);
        Self {
            cookies: vec![access_cookie, refresh_cookie],
            ..self
        }
    }
}

impl<T> Responder for ApiResponse<T>
where
    T: Serialize,
{
    fn respond_to(self, _: &web::HttpRequest) -> impl Future<Output = web::HttpResponse> {
        let body = serde_json::to_string(&self).expect("Serialization failed");
        ready({
            let mut builder = HttpResponse::build(
                StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::OK),
            );
            for cookie in self.cookies {
                builder.cookie(cookie);
            }
            builder.content_type("application/json").body(body)
        })
    }
}
