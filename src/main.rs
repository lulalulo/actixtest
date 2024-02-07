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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpSErver::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
