use actix_cors::Cors;
use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
use serde_json::Result;

mod models;
mod lowmaf;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("lowmaf actix-web server is live!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
              .allow_any_origin()
              .allow_any_header()
              .allow_any_method();
        let json_config = web::JsonConfig::default()
            //.limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });
        App::new()
            .wrap(cors)
            .service(
                web::resource("/api/analyze/0/")
                    .app_data(json_config)
                    //.route(web::post().to(receive_data)),
            ).service(
                web::resource("/")
                    .route(web::get().to(index)),
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
