use crate::schema::users;
use diesel::prelude::*;
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
