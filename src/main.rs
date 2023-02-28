mod config;
mod db_setup;
mod routes;

use actix_settings::ApplySettings;
use actix_web::{App, HttpServer};
use log::{error, info, trace};

const APP_TITLE: &str = "SIMPLE API";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    trace!("Starting {}", APP_TITLE);

    let establish_db_connection = db_setup::establish_db_connection();

    match establish_db_connection {
        Ok(_con) => info!("Successfully connected to the DB"),
        Err(error) => error!("Unable to connect the DB {}", error),
    }

    HttpServer::new(|| App::new().configure(routes::init_routes))
        .apply_settings(&config::config())
        .run()
        .await
}
