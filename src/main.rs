use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}
async fn index() {

}
