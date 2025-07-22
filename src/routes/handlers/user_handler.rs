use actix_web::{get, web, Responder};
use crate::utils;

#[get("/{id}")]
pub async fn get_user(id: web::Path<String>) -> impl Responder {
    utils::api_response::ApiResponse::<()>::new(200, format!("Hello, {}!", id), None, None)
}