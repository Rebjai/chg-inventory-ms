#[derive(Debug, Serialize, Deserialize)]
struct WarehouseStock {
    id: u32,
    item_id: u32,
    warehouse_id: u32,
    quantity: u32,
    threshold: u32,
}
