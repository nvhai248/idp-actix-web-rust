use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};

pub struct ApiResponse {
    pub status_code: StatusCode,
    pub body: String,
}

impl ApiResponse {
    pub fn new(status_code: u16, body: String) -> Self {
        let status = StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        ApiResponse {
            status_code: status,
            body,
        }
    }
}

impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::build(self.status_code)
            .content_type("application/json")
            .body(self.body)
    }
}
