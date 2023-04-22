mod api;
mod config;
mod db_setup;
mod models;
mod schema;

use actix_settings::ApplySettings;
use actix_web::{web::Data, App, HttpServer};
use log::trace;

const APP_TITLE: &str = "SIMPLE API";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    trace!("Starting {}", APP_TITLE);

    let db_pool = Data::new(db_setup::establish_db_connection());

    HttpServer::new(move || {
        App::new()
            .configure(api::init_routes)
            .app_data(db_pool.clone())
    })
    .apply_settings(&config::config())
    .run()
    .await
}
