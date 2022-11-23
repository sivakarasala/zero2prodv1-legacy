//! lib.rs

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new( || {
            App::new()
                .route("/health_check", web::get().to(health_check))
                // A new entry in our routing table for POST /subscritpionss requests
                .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run();
    Ok(server)
}