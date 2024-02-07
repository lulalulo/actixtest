use actix_web::{get, web, App, HttpServer};

// This represents state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; //GET app_name
    format!("Hello {app_name}") //response with app_name
}

