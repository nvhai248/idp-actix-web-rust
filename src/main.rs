mod routes;
mod utils;

use actix_web::{App, HttpServer, middleware::Logger};
use std::env;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        unsafe {
            env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let port = (utils::constants::PORT).clone();
    let address = (utils::constants::ADDRESS).clone();

    println!("Port: {}", port);
    println!("Address: {}", address);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}
