use crate::models::users::NewUser;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;

#[get("/")]
async fn root_path(val: String) -> impl Responder {
    HttpResponse::Ok().body("root loaded")
}

#[post("/users")]
async fn users_path(pool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    use crate::schema::users;

    let conections_pool = pool.get();

    if let Err(e) = conections_pool {
        return HttpResponse::InternalServerError().body(format!("{}", e));
    }

    let mut con = conections_pool.unwrap();

    let user = NewUser {
        user_id: 1,
        user_name: "test 1".into(),
        email: "test@teset.com".into(),
    };

    // let values: Vec<User> =
    let row_inserted = diesel::insert_into(users::table)
        .values(&user)
        .execute(&mut con)
        .expect("user inserting failed");

    HttpResponse::Ok().body("")
}

#[get("/users/{userId}")]
async fn user_path(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
    let user_id = pool.get();
    Ok(HttpResponse::Ok().json(""))
}

#[post("/books")]
async fn books_path() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(""))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(user_path);
    config.service(users_path);
    config.service(root_path);
    config.service(books_path);
}
