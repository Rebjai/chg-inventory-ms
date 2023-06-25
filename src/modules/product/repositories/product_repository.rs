use diesel::prelude::*;
use diesel::PgConnection;

use crate::modules::product::models::Product;
use chg_inventory_ms::schema::products;

pub struct ProductRepository<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> ProductRepository<'_> {
    pub fn new(connection: &mut PgConnection) -> Self {
        ProductRepository { connection }
    }

    pub async fn create_product(&self, product: &Product) -> Result<Product, Box<dyn std::error::Error>> {
        let inserted_product: Product = diesel::insert_into(products::table)
            .values(product)
            .get_result(&self.connection)?;
        Ok(inserted_product)
    }
}