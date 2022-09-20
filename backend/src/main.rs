use actix_cors::Cors;
use actix_web::web::{scope, Data};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use log::{error, info};

use stores::auth::AuthStore;

use crate::routes::auth_scope;
use crate::utils::get_env_port;

mod routes;

pub mod middleware;
pub mod models;
pub mod stores;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("Loaded environment variables");

    let port = get_env_port();
    let auth_store = AuthStore::create();

    info!("Drive-PI starting on port {} if you are", port);
    info!("running this on the Raspberry PI access point ");
    if port == 80 {
        info!("you can access it through http://drivepi.local");
    } else {
        info!("you can access it through http://drivepi.local:{}", port);
    }

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        let auth_store_data = Data::new(auth_store.clone());
        App::new()
            .wrap(cors)
            .app_data(auth_store_data)
            .service(
                scope("/api").configure(routes::auth::init_routes).service(
                    auth_scope(auth_store.clone())
                        .configure(routes::drives::init_routes)
                        .configure(routes::files::init_routes),
                ),
            )
            .configure(routes::app::init_routes)
    });

    server.bind(("0.0.0.0", port))?.run().await
}
