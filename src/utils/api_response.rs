use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct BodyResponse<T: Serialize> {
    pub message: String,
    pub status_code: u16,
    pub error: Option<String>,
    pub data: Option<T>,
}

pub struct ApiResponse<T: Serialize> {
    pub status_code: StatusCode,
    pub body: BodyResponse<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(status_code: u16, message: String, data: Option<T>, error: Option<String>) -> Self {
        let status = StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = BodyResponse {
            message,
            status_code,
            error,
            data,
        };
        ApiResponse {
            status_code: status,
            body,
        }
    }
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = match serde_json::to_string(&self.body) {
            Ok(json) => json,
            Err(_) => "{\"message\": \"Failed to serialize response\"}".to_string(),
        };

        HttpResponse::build(self.status_code)
            .content_type("application/json")
            .body(body)
    }
}
