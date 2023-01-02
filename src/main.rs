use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
use serde_json::Result;

mod models;
mod lowmaf;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("lowmaf actix-web server is live!")
}

async fn analyze() -> impl Responder {
    HttpResponse::Ok().body("Analyze function is working")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .route("/analyze/0/", web::get().to(analyze))
            )
                // using scope allows us to more easily specify extra routes in the future
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
