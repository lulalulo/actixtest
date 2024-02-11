use actix_web::{
    get, http::header::ContentEncoding, middleware, App, HttpResponse, HttpServer,
};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .inser_header(ContentEncoding::Identity)
        .body("data")
}

