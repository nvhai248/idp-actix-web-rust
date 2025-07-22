use crate::utils;
use crate::utils::app_state::AppState;
use actix_web::{Responder, delete, get, put, web};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[get("/{id}")]
pub async fn get_user(app_state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let db = &app_state.db;
    let user_id: i32 = id.parse().unwrap();
    match entity::user::Entity::find_by_id(user_id).one(db).await {
        Ok(Some(user)) => utils::api_response::ApiResponse::<entity::user::Model>::new(
            200,
            "OK".to_string(),
            Some(user),
            None,
        ),
        Ok(None) => {
            utils::api_response::ApiResponse::new(404, "User not found".to_string(), None, None)
        }
        Err(err) => utils::api_response::ApiResponse::new(500, err.to_string(), None, None),
    }
}

#[get("")]
pub async fn get_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let db = &app_state.db;

    match entity::user::Entity::find().all(db).await {
        Ok(users) => {
            if users.is_empty() {
                utils::api_response::ApiResponse::new(404, "User not found".to_string(), None, None)
            } else {
                utils::api_response::ApiResponse::<Vec<entity::user::Model>>::new(
                    200,
                    "OK".to_string(),
                    Some(users),
                    None,
                )
            }
        }
        Err(err) => utils::api_response::ApiResponse::new(500, err.to_string(), None, None),
    }
}

#[put("/{id}")]
pub async fn update_user(
    app_state: web::Data<AppState>,
    id: web::Path<i32>,
    payload: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let db = &app_state.db;

    match entity::user::Entity::find_by_id(id.into_inner())
        .one(db)
        .await
    {
        Ok(Some(user)) => {
            let mut active: entity::user::ActiveModel = user.into();

            if let Some(name) = &payload.name {
                active.name = Set(name.clone());
            }
            if let Some(email) = &payload.email {
                active.email = Set(email.clone());
            }

            match active.update(db).await {
                Ok(updated) => utils::api_response::ApiResponse::<entity::user::Model>::new(
                    200,
                    "User updated".to_string(),
                    Some(updated),
                    None,
                ),
                Err(err) => utils::api_response::ApiResponse::new(500, err.to_string(), None, None),
            }
        }
        Ok(None) => {
            utils::api_response::ApiResponse::new(404, "User not found".to_string(), None, None)
        }
        Err(err) => utils::api_response::ApiResponse::new(500, err.to_string(), None, None),
    }
}

#[delete("/{id}")]
pub async fn delete_user(app_state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let db = &app_state.db;
    let user_id: i32 = match id.parse() {
        Ok(v) => v,
        Err(_) => {
            return utils::api_response::ApiResponse::<()>::new(
                400,
                "Invalid user id".to_string(),
                None,
                None,
            );
        }
    };

    match entity::user::Entity::delete_by_id(user_id).exec(db).await {
        Ok(res) => {
            if res.rows_affected == 0 {
                utils::api_response::ApiResponse::<()>::new(
                    404,
                    "User not found".to_string(),
                    None,
                    None,
                )
            } else {
                utils::api_response::ApiResponse::<()>::new(200, "Deleted".to_string(), None, None)
            }
        }
        Err(err) => utils::api_response::ApiResponse::new(500, err.to_string(), None, None),
    }
}
