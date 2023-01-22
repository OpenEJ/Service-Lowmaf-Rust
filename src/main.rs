use actix_cors::Cors;
use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
use serde_json::Result;

mod models;
mod lowmaf;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("lowmaf actix-web server is live!")
}

// extract json body and deserialize into LowMafInput struct
// send Vec<LowMafInput> to begin() function located in lowmaf.rs
async fn analyze(input: web::Json<Vec<models::LowmafInput>>) -> Result<impl Responder> {
    let resp = lowmaf::begin(input.into_inner());
    return serde_json::to_string(&resp);
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
              // .allow_any_origin()
              .allow_any_header()
              .allow_any_method();
        let json_config = web::JsonConfig::default()
            //.limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                    .into()
            });
        App::new().service(
            web::resource("/api/analyze/0/")
                .app_data(json_config)
                .route(web::post().to(analyze)),
        ).service(
            web::resource("/heartbeat/0/")
                .route(web::get().to(index))
        )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
