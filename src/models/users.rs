use crate::schema::users;
use actix_web::web;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub email: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub user_id: i32,
    pub user_name: String,
    pub email: String,
}

pub struct UserOperations;

impl UserOperations {
    pub fn new_user(
        new_user: web::Json<NewUser>,
        con: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<(), std::fmt::Error> {
        diesel::insert_into(users::table)
            .values(new_user.into_inner())
            .execute(con)
            .expect("error creating user");
        Ok(())
    }
}
