use actix_web::{get, middleware, App, HttpResponse, HttpServer};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("data")
}


