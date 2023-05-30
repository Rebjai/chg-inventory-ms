use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    id: u32,
    name: String,
    price: f64,
}
#[derive(Debug, Serialize, Deserialize)]
struct Wharehouse {
    id: u32,
    name: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Area {
    id: u32,
    name: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct WarehouseStock {
    id: u32,
    item_id: u32,
    warehouse_id: u32,
    quantity: u32,
    threshold: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct AreaStock {
    id: u32,
    item_id: u32,
    area_id: u32,
    quantity: u32,
    threshold: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct InventoryLog {
    id: u32,
    area_id: u32,
    item_id: u32,
    quantity: u32,
    price: f64,
    timestamp: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct SupplyLog {
    id: u32,
    area_id: u32,
    item_id: u32,
    quantity: u32,
    price: f64,
    timestamp: String,
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:8000")? // Replace with your desired host and port
    .run()
    .await
}