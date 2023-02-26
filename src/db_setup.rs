use diesel::pg::PgConnection;
use diesel::Connection;

use crate::config;

pub fn establish_db_connection() -> PgConnection {
    let configs = config::application_config();
    let db_url = configs.application.db_url;
    PgConnection::establish(&db_url).unwrap()
}
