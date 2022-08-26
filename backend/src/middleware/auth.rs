use std::fmt;
use std::future::{ready, Ready};
use std::rc::Rc;

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, HttpResponse};
use actix_web::http::header::HeaderValue;
use derive_more::Display;
use futures::future::LocalBoxFuture;
use futures::FutureExt;

use crate::stores::auth::AuthStoreSafe;

const TOKEN_HEADER: &str = "X-Token";

pub struct AuthMiddleware {
    auth_store: AuthStoreSafe,
}

impl AuthMiddleware {
    pub fn new(auth_store: AuthStoreSafe) -> Self {
        Self { auth_store }
    }
}


// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=actix_web::Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Transform = AuthMiddlewareInner<S>;
    type InitError = ();
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
    auth_store: AuthStoreSafe,
}

impl<S> AuthMiddlewareInner<S> {
    fn check_token(&self, req: &ServiceRequest) -> AuthResult<bool> {
        let headers = req.headers();
        match headers.get(TOKEN_HEADER) {
            Some(token) => {
                let token = token.to_str()
                    .map_err(|_| AuthError::InternalServerError)?
                    .to_string();
                let mut auth_store = self.auth_store.lock()
                    .map_err(|_| AuthError::InternalServerError)?;
                let is_valid = auth_store.check_token(&token)
                    .map_err(|_| AuthError::InternalServerError)?;
                Ok(is_valid)
            }
            None => Err(AuthError::MissingToken)
        }
    }
}

/// Errors for the authentication endpoint
#[derive(Debug, Display)]
pub enum AuthError {
    #[display(fmt = "missing token")]
    MissingToken,
    #[display(fmt = "invalid token")]
    InvalidToken,
    #[display(fmt = "internal server error")]
    InternalServerError,
}

type AuthResult<T> = Result<T, AuthError>;


fn e500<E: fmt::Debug + fmt::Display + 'static>(err: E) -> actix_web::Error {
    actix_web::error::InternalError::from_response(
        err,
        HttpResponse::InternalServerError().finish(),
    )
        .into()
}

fn e401<E: fmt::Debug + fmt::Display + 'static>(err: E) -> actix_web::Error {
    actix_web::error::InternalError::from_response(
        err,
        HttpResponse::Unauthorized().finish(),
    )
        .into()
}


impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=actix_web::Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let valid = self.check_token(&req);
        async move {
            match valid {
                Ok(valid_token) => {
                    if valid_token {
                        service.call(req).await
                    } else {
                        Err(e401(AuthError::InvalidToken))
                    }
                }
                Err(e) => {
                    match e {
                        AuthError::InternalServerError => Err(e500(e)),
                        e => Err(e401(e))
                    }
                }
            }
        }.boxed_local()
    }
}