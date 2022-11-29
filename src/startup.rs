//! src/startup.rs

use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgConnection;

use crate::routes::{health_check, subscribe};


pub fn run(
  listener: TcpListener,
  // New parameter!
  connection: PgConnection
) -> Result<Server, std::io::Error> {
  // Wrap the connection in a smart pointer
  let connection = web::Data::new(connection);
  let server = HttpServer::new( || {
          App::new()
              .route("/health_check", web::get().to(health_check))
              // A new entry in our routing table for POST /subscritpionss requests
              .route("/subscriptions", web::post().to(subscribe))
              // Register the connection as part of the application state
              // Get a pointer copy and attach it to the application state
              .app_data(connection.clone())
      })
      .listen(listener)?
      .run();
  Ok(server)
}