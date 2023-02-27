mod config;
mod db_setup;
mod routes;

use actix_settings::ApplySettings;
use actix_web::{App, HttpServer};
use log::{error, trace};

const APP_TITLE: &str = "SIMPLE API";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    trace!("Starting {}", APP_TITLE);

    let establish_db_connection = db_setup::establish_db_connection();

    if establish_db_connection.is_err() {
        error!("unable to connect to db")
    }

    HttpServer::new(|| App::new().configure(routes::init_routes))
        .apply_settings(&config::config())
        .run()
        .await
}
