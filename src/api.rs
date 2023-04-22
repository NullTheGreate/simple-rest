use actix_web::{get, post, web, Error, HttpResponse, Responder};
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;
use Simple_Rest::models::users::UserOperations;

#[get("/users")]
async fn root_path(val: String) -> impl Responder {
    HttpResponse::Ok().body("root loaded")
}

#[post("/users")]
async fn users_path(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    new_user: web::Json<Simple_Rest::models::users::NewUser>,
) -> impl Responder {
    let mut con = pool.get().unwrap();

    let _ = UserOperations::new_user(new_user, &mut con);

    HttpResponse::Ok().json(true)
}

#[get("/users/{userId}")]
async fn user_path(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, Error> {
    let user_id = pool.get();

    // if let Err(e) = user_id {
    //     return Err(e);
    // }

    let mut con = user_id.unwrap();

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
