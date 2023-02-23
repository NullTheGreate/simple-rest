mod config;
mod routes;

use actix_settings::ApplySettings;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::init_routes))
        .apply_settings(&config::config())
        .run()
        .await
}
