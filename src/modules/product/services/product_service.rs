use crate::modules::product::models::Product;
use crate::modules::product::repositories::ProductRepository;

pub struct ProductService {
    product_repository: ProductRepository,
}

impl ProductService {
    pub fn new(product_repository: ProductRepository) -> Self {
        ProductService { product_repository }
    }

    pub async fn create_product(&self, name: &str, price: f32) -> Result<Product, Box<dyn std::error::Error>> {
        let new_product = Product::new(name.to_owned(), price);
        self.product_repository.create_product(&new_product).await
    }
}
