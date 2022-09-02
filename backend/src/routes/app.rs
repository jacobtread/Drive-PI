use actix_web::{get, HttpResponse, web};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use mime_guess::from_path;
use rust_embed::{EmbeddedFile, RustEmbed};

#[derive(RustEmbed)]
#[folder = "public"]
struct PublicDir;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(public);
}

async fn serve_file(path: &str, file: EmbeddedFile) -> HttpResponse<BoxBody> {
    HttpResponse::Ok()
        .content_type(from_path(path)
            .first_or_octet_stream()
            .as_ref())
        .body(file.data.into_owned())
}

#[get("/{filename:.*}")]
async fn public(path: web::Path<String>) -> HttpResponse<BoxBody> {
    if let Some(file) = PublicDir::get(&path) {
        serve_file(&path, file).await
    } else if let Some(file) = PublicDir::get("index.html") {
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(file.data.into_owned())
    } else {
        HttpResponse::NotFound()
            .body("404 Not Found")
    }
}

