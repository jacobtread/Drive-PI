use actix_web::{App, get, HttpResponse, HttpServer, post, Responder};
use actix_web::web::{Data, Json};
use dotenv::dotenv;
use log::info;
use serde::Serialize;

use stores::auth::AuthStore;

mod routes;

pub mod utils;
pub mod stores;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv()
        .ok();
    env_logger::init();

    info!("Loaded environment variables");


    let auth_store = AuthStore::create()
        .to_safe();

    HttpServer::new(move || {
        let auth_store_data = Data::new(auth_store.clone());
        App::new()
            .app_data(auth_store_data)
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
