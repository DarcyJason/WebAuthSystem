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
}

impl<T> ApiResponse<T> {
    pub fn success(msg: &str, data: T) -> Self {
        Self {
            status: "success".to_string(),
            status_code: StatusCode::OK.as_u16(),
            message: msg.to_string(),
            data: Some(data),
        }
    }

    pub fn error(msg: &str, status: StatusCode) -> Self {
        Self {
            status: "error".to_string(),
            status_code: status.as_u16(),
            message: msg.to_string(),
            data: None,
        }
    }
}

impl<T> Responder for ApiResponse<T>
where
    T: Serialize,
{
    fn respond_to(self, _: &web::HttpRequest) -> impl Future<Output = web::HttpResponse> {
        let body = match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(_) => {
                return ready(
                    HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .body(r#"{"status":"error","status_code":"500",message":"Internal Server Error","data":""}"#),
                );
            }
        };

        ready(
            HttpResponse::build(
                StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            )
            .content_type("application/json")
            .body(body),
        )
    }
}
