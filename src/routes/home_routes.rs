use crate::routes::handlers;
use crate::routes::middlewares::auth_middleware::JwtMiddleware;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/home")
            .service(handlers::home_handler::test)
            .service(
                web::scope("")
                    .wrap(JwtMiddleware)
                    .service(handlers::home_handler::greet),
            ),
    );
}
