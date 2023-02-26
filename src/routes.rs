use actix_web::{get, post, web, Error, HttpResponse, Responder};

#[get("/")]
async fn root_path() -> impl Responder {
    HttpResponse::Ok().body("root loaded")
}

#[post("/users")]
async fn users_path(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/users/{userId}")]
async fn user_path(path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();
    Ok(HttpResponse::Ok().json(user_id))
}

#[get("/books")]
async fn books_path() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(""))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(user_path);
    config.service(users_path);
    config.service(root_path);
    config.service(books_path);
}
