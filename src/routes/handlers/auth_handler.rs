// routes/auth.rs
use crate::utils::app_state::AppState;
use crate::utils::auth::{Claims, LoginRequest, RegisterRequest};
use crate::utils::{self, hash::hash_password};
use actix_web::{Responder, post, web, get};
use entity::user::ActiveModel;
use jsonwebtoken::{DecodingKey, Validation};
use sea_orm::{ActiveValue::Set, entity::prelude::*};

#[post("/login")]
pub async fn login(
    app_state: web::Data<AppState>,
    payload: web::Json<LoginRequest>,
) -> impl Responder {
    let db = &app_state.db;

    // Find user by email
    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(&payload.email))
        .one(db)
        .await;

    match user {
        Ok(Some(user_model)) => {
            // Verify password
            let is_valid =
                match utils::hash::verify_password(&user_model.password, &payload.password) {
                    Ok(valid) => valid,
                    Err(_) => false,
                };

            if is_valid {
                // Create JWT
                let jwt_result =
                    utils::auth::create_jwt(&user_model.id.to_string(), &user_model.email);

                match jwt_result {
                    Ok(token) => {
                        // Decode token to get exp
                        let decoded = jsonwebtoken::decode::<Claims>(
                            &token,
                            &DecodingKey::from_secret(utils::constants::JWT_SECRET.as_ref()),
                            &Validation::default(),
                        );

                        let expires_at = match decoded {
                            Ok(data) => data.claims.exp,
                            Err(_) => 0,
                        };

                        let response_data = serde_json::json!({
                            "token": token,
                            "expires_at": expires_at,
                        });

                        utils::api_response::ApiResponse::new(
                            200,
                            "Login successful".to_string(),
                            Some(response_data),
                            None,
                        )
                    }
                    Err(_) => utils::api_response::ApiResponse::new(
                        500,
                        "Failed to generate token".to_string(),
                        None,
                        None,
                    ),
                }
            } else {
                utils::api_response::ApiResponse::new(
                    401,
                    "Invalid credentials".to_string(),
                    None,
                    None,
                )
            }
        }
        Ok(None) => utils::api_response::ApiResponse::new(
            401,
            "Invalid credentials".to_string(),
            None,
            None,
        ),
        Err(_) => {
            utils::api_response::ApiResponse::new(500, "Database error".to_string(), None, None)
        }
    }
}

#[post("/register")]
pub async fn register(
    app_state: web::Data<AppState>,
    payload: web::Json<RegisterRequest>,
) -> impl Responder {
    let db = &app_state.db;

    let hashed_password = match hash_password(&payload.password) {
        Ok(h) => h,
        Err(_) => {
            return utils::api_response::ApiResponse::<()>::new(
                500,
                "Failed to hash password".to_string(),
                None,
                None,
            );
        }
    };

    let user = ActiveModel {
        name: Set(payload.name.clone()),
        email: Set(payload.email.clone()),
        password: Set(hashed_password),
        ..Default::default()
    };

    let insert_result = user.insert(db).await;

    match insert_result {
        Ok(_model) => utils::api_response::ApiResponse::<()>::new(
            201,
            "User registered successfully".to_string(),
            None,
            None,
        ),
        Err(_) => utils::api_response::ApiResponse::new(
            500,
            "Failed to register user".to_string(),
            None,
            None,
        ),
    }
}

#[get("/profile")]
pub async fn get_profile(app_state: web::Data<AppState>, claims: Claims) -> impl Responder {
    let db = &app_state.db;

    let user_id: i32 = match claims.sub.parse() {
        Ok(id) => id,
        Err(_) => {
            return utils::api_response::ApiResponse::new(
                400,
                "Invalid user ID in token".to_string(),
                None,
                None,
            );
        }
    };

    match entity::user::Entity::find_by_id(user_id).one(db).await {
        Ok(Some(user)) => utils::api_response::ApiResponse::<entity::user::Model>::new(
            200,
            "OK".to_string(),
            Some(user),
            None,
        ),
        Ok(None) => utils::api_response::ApiResponse::new(
            404,
            "User not found".to_string(),
            None,
            None,
        ),
        Err(err) => utils::api_response::ApiResponse::new(
            500,
            err.to_string(),
            None,
            None,
        ),
    }
}
