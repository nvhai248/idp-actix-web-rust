use crate::routes::handlers;
use crate::routes::middlewares::auth_middleware::JwtMiddleware;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/users")
            .wrap(JwtMiddleware)
            .service(handlers::user_handler::get_user),
    );
}
