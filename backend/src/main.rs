mod stores;
mod routes;

use std::env::VarError;
use std::sync::{Arc, Mutex};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Json;
use serde::Serialize;
use dotenv::dotenv;
use crate::stores::auth::AuthStore;

const DEFAULT_USERNAME: &str = "admin";
const DEFAULT_PASSWORD: &str = "admin";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv()
        .ok();

    env_logger::init();

    let username = match std::env::var("DRIVEPI_USERNAME") {
        Ok(value) => value,
        Err(_) => DEFAULT_USERNAME.to_string()
    };
    let password = match std::env::var("DRIVEPI_PASSWORD") {
        Ok(value) => value,
        Err(_) => DEFAULT_PASSWORD.to_string()
    };
    let auth_store = Arc::new(
        Mutex::new(
            AuthStore::new(username, password)
        )
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(auth_store.clone()))
            .service(routes::auth::auth)
            .service(hello)
            .service(echo)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[derive(Serialize)]
struct TestResponse {
    message: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    Json(TestResponse {
        message: "Hello World".to_string()
    })
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}