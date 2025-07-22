use actix_web::{FromRequest, HttpRequest, dev::Payload};
use chrono::Utc;
use futures_util::future::{Ready, ready};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode, errors::Error,
};
use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // Subject (user ID)
    pub exp: usize,    // Expiration time
    pub iat: usize,    // Issued at
    pub email: String, // User email
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, actix_web::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..]; // Skip "Bearer "
                    match verify_jwt(token) {
                        Ok(data) => return ready(Ok(data.claims)),
                        Err(_) => {
                            return ready(Err(actix_web::error::ErrorUnauthorized(
                                "Invalid token",
                            )));
                        }
                    }
                }
            }
        }
        ready(Err(actix_web::error::ErrorUnauthorized(
            "Missing Authorization header",
        )))
    }
}

pub fn create_jwt(user_id: &str, email: &str) -> Result<String, Error> {
    let secret = utils::constants::JWT_SECRET.clone();

    let now = Utc::now().timestamp() as usize;
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        email: email.to_owned(),
        iat: now,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, Error> {
    let secret = utils::constants::JWT_SECRET.clone();

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}
