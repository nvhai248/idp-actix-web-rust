use crate::routes::handlers;
use crate::routes::middlewares::auth_middleware::JwtMiddleware;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/auth")
            .service(handlers::auth_handler::register)
            .service(handlers::auth_handler::login)
            .service(
                web::scope("/")
                    .wrap(JwtMiddleware)
                    .service(handlers::auth_handler::get_profile),
            ),
    );
}
