use std::env;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref JWT_SECRET: String = set_jwt_secret();
    pub static ref PORT: u16 = set_port();
}

fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string())
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    env::var("PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(8080)
}

fn set_database_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL")
        .unwrap_or_else(|_| "Not found".to_string())
}

fn set_jwt_secret() -> String {
    dotenv::dotenv().ok();
    env::var("JWT_SECRET")
        .unwrap_or_else(|_| "Not found".to_string())
}
