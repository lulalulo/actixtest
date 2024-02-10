use actix_web::{get, web, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    user_id, u32,
    friend: String,
}

#[get("/users/{users_id}/{friend}")] // <- define path params
async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.info_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
