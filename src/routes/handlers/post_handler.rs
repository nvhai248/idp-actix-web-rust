use crate::utils;
use crate::utils::app_state::AppState;
use crate::utils::auth::Claims;
use actix_web::{Responder, delete, get, post, put, web};
use entity::post::ActiveModel as PostActiveModel;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub text: String,
}

#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub text: Option<String>,
}

#[get("")]
pub async fn get_own_post(app_state: web::Data<AppState>, claims: Claims) -> impl Responder {
    let db = &app_state.db;
    let user_id: i32 = claims.sub.parse().unwrap_or_default();

    match entity::post::Entity::find()
        .filter(entity::post::Column::UserId.eq(user_id))
        .all(db)
        .await
    {
        Ok(posts) if posts.is_empty() => {
            utils::api_response::ApiResponse::new(404, "No posts found".to_string(), None, None)
        }
        Ok(posts) => utils::api_response::ApiResponse::<Vec<entity::post::Model>>::new(
            200,
            "OK".to_string(),
            Some(posts),
            None,
        ),
        Err(err) => utils::api_response::ApiResponse::new(500, err.to_string(), None, None),
    }
}

#[post("")]
pub async fn create_post(
    app_state: web::Data<AppState>,
    payload: web::Json<CreatePostRequest>,
    claims: Claims,
) -> impl Responder {
    let db = &app_state.db;

    let user_id: i32 = claims.sub.clone().parse().unwrap();

    let post = PostActiveModel {
        title: Set(payload.title.clone()),
        text: Set(payload.text.clone()),
        user_id: Set(user_id), // Assuming `sub` is user_id from token
        ..Default::default()
    };

    match post.insert(db).await {
        Ok(created_post) => utils::api_response::ApiResponse::<entity::post::Model>::new(
            201,
            "Post created successfully".to_string(),
            Some(created_post),
            None,
        ),
        Err(err) => utils::api_response::ApiResponse::new(500, err.to_string(), None, None),
    }
}

#[put("/{id}")]
pub async fn update_post(
    app_state: web::Data<AppState>,
    id: web::Path<i32>,
    payload: web::Json<UpdatePostRequest>,
    claims: Claims,
) -> impl Responder {
    let db = &app_state.db;
    let user_id: i32 = claims.sub.parse().unwrap_or_default();

    match entity::post::Entity::find_by_id(id.into_inner())
        .one(db)
        .await
    {
        Ok(Some(post)) => {
            // Check ownership
            if post.user_id != user_id {
                return utils::api_response::ApiResponse::<entity::post::Model>::new(
                    403,
                    "You do not have permission to update this post".to_string(),
                    None,
                    None,
                );
            }

            let mut active: entity::post::ActiveModel = post.into();

            if let Some(title) = &payload.title {
                active.title = Set(title.clone());
            }

            if let Some(text) = &payload.text {
                active.text = Set(text.clone());
            }

            match active.update(db).await {
                Ok(updated_post) => utils::api_response::ApiResponse::<entity::post::Model>::new(
                    200,
                    "Post updated successfully".to_string(),
                    Some(updated_post),
                    None,
                ),
                Err(err) => utils::api_response::ApiResponse::new(500, err.to_string(), None, None),
            }
        }
        Ok(None) => {
            utils::api_response::ApiResponse::new(404, "Post not found".to_string(), None, None)
        }
        Err(err) => utils::api_response::ApiResponse::new(500, err.to_string(), None, None),
    }
}

#[delete("/{id}")]
pub async fn delete_post(
    app_state: web::Data<AppState>,
    id: web::Path<i32>,
    claims: Claims,
) -> impl Responder {
    use sea_orm::ActiveModelTrait;

    let db = &app_state.db;
    let user_id: i32 = claims.sub.parse().unwrap_or_default();

    match entity::post::Entity::find_by_id(id.into_inner())
        .one(db)
        .await
    {
        Ok(Some(post)) => {
            // Check ownership
            if post.user_id != user_id {
                return utils::api_response::ApiResponse::<()>::new(
                    403,
                    "You do not have permission to delete this post".to_string(),
                    None,
                    None,
                );
            }

            // Convert to ActiveModel to delete
            let active_post: entity::post::ActiveModel = post.into();

            match active_post.delete(db).await {
                Ok(_) => utils::api_response::ApiResponse::<()>::new(
                    200,
                    "Post deleted successfully".to_string(),
                    None,
                    None,
                ),
                Err(err) => utils::api_response::ApiResponse::new(500, err.to_string(), None, None),
            }
        }
        Ok(None) => {
            utils::api_response::ApiResponse::new(404, "Post not found".to_string(), None, None)
        }
        Err(err) => utils::api_response::ApiResponse::new(500, err.to_string(), None, None),
    }
}
