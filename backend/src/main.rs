use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Json;
use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Serialize)]
struct TestResponse {
    message: String
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