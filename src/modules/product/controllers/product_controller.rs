use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::modules::product::product_service::ProductService;

#[derive(Debug, Serialize, Deserialize)]
struct CreateProductRequest {
    name: String,
    price: f32,
}

async fn create_product(
    product_data: web::Json<CreateProductRequest>,
    product_service: web::Data<ProductService>,
) -> impl Responder {
    let new_product = product_service.create_product(&product_data.name, product_data.price).await;
    match new_product {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/products").route(web::post().to(create_product)));
}
