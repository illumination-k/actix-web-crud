#[macro_use]
extern crate diesel;

use anyhow::Result;

use actix_web::{get, App, HttpServer, Responder};

mod database;
mod models;
mod routes;
mod schema;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    let pool = database::establish_connection()?;
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(hello)
            .service(routes::posts::get::index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .expect("Error in build httpserver");
    Ok(())
}
