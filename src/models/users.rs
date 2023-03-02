use crate::schema::users;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub user_id: &'a i32,
    pub user_name: &'a str,
    pub email: &'a str,
}
