use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

use crate::config;

pub fn establish_db_connection() -> Pool<ConnectionManager<PgConnection>> {
    let configs = config::application_config();
    let db_url: String = configs.application.db_url;

    Pool::builder()
        .build(ConnectionManager::new(db_url))
        .expect("failed to connect to db")
}
