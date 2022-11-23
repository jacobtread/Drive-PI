use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::LocalBoxFuture;
use futures::FutureExt;
use std::future::{ready, Ready};
use std::rc::Rc;
use std::sync::Arc;

use crate::models::errors::{server_error, AuthError};
use crate::stores::auth::AuthStore;

pub const TOKEN_HEADER: &str = "X-Token";

/// Struct representing a the base middleware for
/// authentication tokens
pub struct AuthMiddleware {
    auth_store: Arc<AuthStore>,
}

impl AuthMiddleware {
    // Constructor function for creating a new middleware
    pub fn new(auth_store: Arc<AuthStore>) -> Self {
        Self { auth_store }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    // The transform service this middleware creates
    type Transform = AuthMiddlewareInner<S>;
    type InitError = ();
    // The future type that `new_transform` will return when creating
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareInner {
            service: Rc::new(service),
            auth_store: self.auth_store.clone(),
        }))
    }
}

pub struct AuthMiddlewareInner<S> {
    service: Rc<S>,
    auth_store: Arc<AuthStore>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let auth_store = self.auth_store.clone();

        async move {
            let headers = req.headers();
            let token_header = headers.get(TOKEN_HEADER).ok_or(AuthError::MissingToken)?;
            let token = token_header.to_str().map_err(server_error)?;
            let is_valid = auth_store.check_token(token).await;
            if is_valid {
                service.call(req).await
            } else {
                Err(AuthError::InvalidToken.into())
            }
        }
        .boxed_local()
    }
}
