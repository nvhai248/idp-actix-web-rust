use chrono::Utc;
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


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // Subject (user ID)
    pub exp: usize,    // Expiration time
    pub iat: usize,    // Issued at
    pub email: String, // User email
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
