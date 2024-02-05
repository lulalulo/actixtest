use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it
            web::scope("/app")
                // this handles request for 'GET /app/index.html'
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
