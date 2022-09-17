use actix_web::body::BoxBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{Error, Scope};

use crate::middleware::auth::AuthMiddleware;
use crate::stores::auth::AuthStoreSafe;

pub mod app;
pub mod auth;
pub mod drives;
pub mod files;

/// Creates a scope that is protected by the auth store
/// authentication middleware
pub fn auth_scope(
    auth_store: AuthStoreSafe,
) -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<BoxBody>,
        Error = Error,
        InitError = (),
    >,
> {
    Scope::new("").wrap(AuthMiddleware::new(auth_store))
}

#[macro_export]
macro_rules! define_routes {
    ($($route:ident),*) => {
        pub fn init_routes(cfg: &mut web::ServiceConfig) {
            cfg
                $(.service($route))*;
        }
    };
}
