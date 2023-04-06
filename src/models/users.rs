use crate::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub email: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub user_id: i32,
    pub user_name: String,
    pub email: String,
}
