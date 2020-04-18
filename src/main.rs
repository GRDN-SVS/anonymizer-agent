extern crate actix;
extern crate actix_web;
extern crate actix_rt;
extern crate tokio;
extern crate serde;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
extern crate reqwest;

use actix_web::{App, HttpServer};

mod database;
mod handlers;
mod models;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}