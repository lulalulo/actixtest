use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize info from request body, max payload size is 4kb
#[get("/")] // <- define path params
async fn index(info: web::Query<info>) -> impl Responder {
    format!("Welcome {}!", info.username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
            let json_config = web::JsonConfig::default()
                .limit(4096)
                .error_handler(|err, _req| {
                    // create custome error responde
                    error::InternalError::from_response(err, HttpResponde::Conflict().finish())
                        .into
                });
            App::new().service(
                web::resource("/")
                    // change json extractor configurator
                    .app_data(json_config)
                    .route(web::post().to(index)),
            )
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
