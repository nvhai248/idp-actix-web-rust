use crate::utils;
use crate::utils::app_state::AppState;
use actix_web::{Responder, get, web};
use sea_orm::{ConnectionTrait, Statement};

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    utils::api_response::ApiResponse::new(200, format!("Hello {name}!").to_string())
}

#[get("/test")]
pub async fn test(app_state: web::Data<AppState>) -> impl Responder {
    let res = app_state
        .db
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT * FROM user;",
        ))
        .await
        .unwrap();
    utils::api_response::ApiResponse::new(200, "OK".to_string())
}
