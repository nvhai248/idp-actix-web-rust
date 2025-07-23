use crate::routes::handlers;
use crate::routes::middlewares::auth_middleware::JwtMiddleware;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/posts")
            .wrap(JwtMiddleware)
            .service(handlers::post_handler::get_own_post)
            .service(handlers::post_handler::create_post)
            .service(handlers::post_handler::update_post)
            .service(handlers::post_handler::delete_post)
    );
}
