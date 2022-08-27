use actix_web::{App, HttpServer};
use actix_web::web::{Data, scope};
use dotenv::dotenv;
use log::info;

use stores::auth::AuthStore;

mod routes;

pub mod utils;
pub mod stores;
pub mod middleware;
pub mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv()
        .ok();
    env_logger::init();

    info!("Loaded environment variables");

    let auth_store = AuthStore::create()
        .to_safe();

    let server = HttpServer::new(move || {
        let auth_store_data = Data::new(auth_store.clone());
        App::new()
            .app_data(auth_store_data)
            .service(
                scope("/api")
                    .configure(routes::auth::init_routes)
                    .configure(|cfg| routes::drives::init_routes(cfg, auth_store.clone()))
            )
    });

    server.bind(("0.0.0.0", 8080))?
        .run()
        .await
}