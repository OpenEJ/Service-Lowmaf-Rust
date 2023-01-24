// use actix_cors::Cors;
use actix_web::{error, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use serde_json::Result;

mod models;
mod lowmaf;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("lowmaf actix-web server is live!")
}

// extract json body and deserialize into LowMafInput struct
// send Vec<LowMafInput> to begin() function located in lowmaf.rs
async fn analyze(input: web::Json<Vec<models::LowmafInput>>) -> HttpResponse {
    let resp = lowmaf::begin(input.into_inner());
    HttpResponse::Ok().json(&resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configure enviornment variables for logging
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        /*
        let cors = Cors::default()
              // .allow_any_origin()
              .allow_any_header()
              .allow_any_method();
        */
        let logger = Logger::default();
        let json_config = web::JsonConfig::default()
            .limit(100000000) // 100M
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });
        App::new()
            .wrap(logger)
            .service(
                web::resource("/api/analyze/0/")
                    .app_data(json_config)
                    .route(web::post().to(analyze)),
            )
            .service(
                web::resource("/heartbeat/0/")
                    .route(web::get().to(index))
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
