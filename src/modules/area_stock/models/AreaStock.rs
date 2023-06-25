#[derive(Debug, Serialize, Deserialize)]
struct AreaStock {
    id: u32,
    item_id: u32,
    area_id: u32,
    quantity: u32,
    threshold: u32,
}