mod modules;

use crate::modules::product::product_repository::ProductRepository;
use crate::modules::product::product_service::ProductService;
use crate::modules::product::product_controller;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::{pg::PgConnection, connection};
use diesel::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn establish_connection_pool() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use chg_inventory_ms::schema::products::dsl::*;
    // Create a database connection using Diesel
    let pool: PgConnection = establish_connection_pool();

    // Create the product repository
    let product_repository = ProductRepository::new(pool.clone());

    // Create the product service
    let product_service = web::Data::new(ProductService::new(product_repository));

    // Create the Actix web App
    let app = HttpServer::new(move || {
        App::new()
            // Add other middleware and configurations as needed
            .app_data(product_service.clone())
            .configure(product_controller::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run();

    println!("Server is running at http://127.0.0.1:8080");
    app.await
}
