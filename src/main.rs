use actix_web::{web, App, HttpServer, Responder};
use std::cel::Cell;

#[derive(Clone)]
struct AppState {
    count: Cell<usize>,
}

