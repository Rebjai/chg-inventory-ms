#[derive(Debug, Serialize, Deserialize)]
struct InventoryLog {
    id: u32,
    area_id: u32,
    item_id: u32,
    quantity: u32,
    price: f64,
    timestamp: String,
}