mod models;
mod routes;
extern crate actix_web;
extern crate mysql;
extern crate dotenv;
extern crate serde;

use actix_web::{web, App, HttpServer};
use crate::routes::AppRoutes;
use dotenv::dotenv;
use std::env;
use mysql::{Pool, PooledConn};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(AppRoutes::index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

pub fn create_connection() -> Result<PooledConn, mysql::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = Pool::new(database_url).expect("Could not create pool");
    let conn = pool.get_conn().expect("Could not connect");
    Ok(conn)
}
