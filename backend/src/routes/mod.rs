use actix_web::{Error, Scope};
use actix_web::body::BoxBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};

use crate::middleware::auth::AuthMiddleware;
use crate::stores::auth::AuthStoreSafe;

pub mod auth;

/// Creates a scope that is protected by the auth store
/// authentication middleware
pub fn auth_scope(auth_store: AuthStoreSafe) -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config=(),
        Response=ServiceResponse<BoxBody>,
        Error=Error,
        InitError=(),
    >,
> {
    return auth_scope_with_path("", auth_store);
}

/// Creates a scope that is protected by the auth store
/// authentication middleware this scope has a path
pub fn auth_scope_with_path(path: &str, auth_store: AuthStoreSafe) -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config=(),
        Response=ServiceResponse<BoxBody>,
        Error=Error,
        InitError=(),
    >,
> {
    Scope::new(path)
        .wrap(AuthMiddleware::new(auth_store))
}