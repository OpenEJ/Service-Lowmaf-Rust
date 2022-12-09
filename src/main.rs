use actix_cors::Cors;
use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
use serde_json::Result;

mod models;
mod lowmaf;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("tip-in-enrichment actix-web server is live!")
}

fn main() {
    println!("Hello, world!");
}
