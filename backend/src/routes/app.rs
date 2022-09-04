use actix_web::{get, HttpResponse, web};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use mime_guess::from_path;
use rust_embed::{RustEmbed};
use crate::define_routes;

#[derive(RustEmbed)]
#[folder = "public"]
struct PublicDir;

define_routes!(public);

#[get("/{filename:.*}")]
async fn public(path: web::Path<String>) -> HttpResponse<BoxBody> {
    let path: &String = path.as_ref();
    if let Some(file) = PublicDir::get(path) {
        HttpResponse::Ok()
            .content_type(from_path(path)
                .first_or_octet_stream()
                .as_ref())
            .body(file.data.into_owned())
    } else if let Some(file) = PublicDir::get("index.html") {
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(file.data.into_owned())
    } else {
        HttpResponse::NotFound()
            .body("404 Not Found")
    }
}

