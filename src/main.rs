use crate::utils::app_state::AppState;
use actix_web::{App, HttpServer, middleware::Logger, web};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::error::Error;
use std::fmt::Display;

mod routes;
mod utils;

#[derive(Debug)]
struct MainError {
    message: String,
}

impl Display for MainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}
impl Error for MainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[actix_web::main]
async fn main() -> Result<(), MainError> {
    if env::var("RUST_LOG").is_err() {
        unsafe {
            env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let port = (utils::constants::PORT).clone();
    let address = (utils::constants::ADDRESS).clone();
    let database_url = (utils::constants::DATABASE_URL).clone();

    let db: DatabaseConnection =
        Database::connect(database_url)
            .await
            .map_err(|err| MainError {
                message: err.to_string(),
            })?;

    Migrator::up(&db, None).await.map_err(|err| MainError {
        message: err.to_string(),
    })?;

    println!("Port: {}", port);
    println!("Address: {}", address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
            .configure(routes::auth_routes::config)
            .configure(routes::user_routes::config)
    })
    .bind((address, port))
    .map_err(|err| MainError {
        message: err.to_string(),
    })?
    .run()
    .await
    .map_err(|err| MainError {
        message: err.to_string(),
    })?;

    Ok(())
}
