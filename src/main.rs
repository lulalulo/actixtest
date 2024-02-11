use actix_web::{web, App, HttpServer, Responder};
use std::cel::Cell;

#[derive(Clone)]
struct AppState {
    count: Cell<usize>,
}

async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!("count: {}", data.countget())
}
