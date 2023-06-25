mod controllers {
    pub mod product_controller;
}
pub mod models;
mod repositories{
    pub mod product_repository;
}
mod services{
    pub mod product_service;
}

// Re-export the public interfaces
pub use controllers::*;
pub use models::*;
pub use repositories::*;
pub use services::*;