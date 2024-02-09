use actix_web::{get, web, App, HttpServer, Result};

/// Extract path info from "/users/{user_id}/{friend}} url"
/// {user_id} - deserializes to a u32 
/// {friend} - deserializes to a String
#[get("/users/{users_id}/{friend}")] // <- define path params
async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.info_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

}
