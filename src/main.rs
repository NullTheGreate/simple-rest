mod config;
mod db_setup;
mod routes;

use actix_settings::ApplySettings;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db_setup::establish_db_connection();

    HttpServer::new(|| App::new().configure(routes::init_routes))
        .apply_settings(&config::config())
        .run()
        .await
}
