use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::web::{Data, scope};
use dotenv::dotenv;
use log::{info, warn};

use stores::auth::AuthStore;

use crate::routes::auth_scope;
use crate::utils::dnsmasq::setup_dnsmasq;
use crate::utils::hotspot::{Hotspot};

mod routes;

pub mod utils;
pub mod stores;
pub mod middleware;
pub mod models;

const ENV_PORT_KEY: &str = "DRIVEPI_PORT";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv()
        .ok();
    env_logger::init();

    info!("Loaded environment variables");

    let port_raw = std::env::var(ENV_PORT_KEY)
        .unwrap_or(String::from("8080"));

    let port = port_raw.parse::<u16>()
        .unwrap_or_else(|_| {
            warn!("Port provided as {} is not a valid port defaulting to 8080", port_raw);
            8080
        });

    let auth_store = AuthStore::create();

    info!("Drive-PI starting on port {} if you are", port);
    info!("running this on the Raspberry PI access point ");
    if port == 80 {
        info!("you can access it through http://drivepi.local");
    } else {
        info!("you can access it through http://drivepi.local:{}", port);
    }

    // Start hotspot
    let _ = Hotspot::start();
    // Configure domain (drivepi.local)
    setup_dnsmasq();

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        let auth_store_data = Data::new(auth_store.clone());
        App::new()
            .wrap(cors)
            .app_data(auth_store_data)
            .service(
                scope("/api")
                    .configure(routes::auth::init_routes)
                    .service(
                        auth_scope(auth_store.clone())
                            .configure(routes::drives::init_routes)
                            .configure(routes::files::init_routes)
                    )
            )
            .configure(routes::app::init_routes)
    });

    server.bind(("0.0.0.0", port))?
        .run()
        .await
}