use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use futures::future::{LocalBoxFuture, Ready, ok};
use std::rc::Rc;

use crate::utils::auth::verify_jwt;

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct JwtMiddlewareMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            // Extract Authorization header
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..]; // Strip "Bearer "

                        // Verify the token
                        match verify_jwt(token) {
                            Ok(token_data) => {
                                // Insert claims into request extensions
                                req.extensions_mut().insert(token_data.claims);
                                return service.call(req).await;
                            }
                            Err(_) => {
                                return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
                            }
                        }
                    }
                }
            }

            Err(actix_web::error::ErrorUnauthorized(
                "Authorization token missing",
            ))
        })
    }
}
