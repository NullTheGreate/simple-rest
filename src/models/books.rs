use diesel::prelude::*;

#[derive(Queryable)]
pub struct Books {
    pub isbn_no: String,
    pub name: String,
    pub author: String,
}
