extern crate actix;
extern crate actix_rt;
extern crate actix_web;
extern crate serde;
extern crate tokio;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;
extern crate reqwest;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod config;
mod database;
mod handlers;
mod models;

use database::executor::DBExecutor;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let addr = actix::SyncArbiter::start(2, || {
        DBExecutor::new(&env::var("DATABASE_URL").expect("No DATABASE_URL in .env"))
    });

    HttpServer::new(move || {
        App::new()
            .data(config::State { db: addr.clone() })
            .service(handlers::nonce::get_nonce)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
