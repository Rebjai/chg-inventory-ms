use serde::{Deserialize, Serialize};
use diesel::{prelude::*, data_types::{PgNumeric, PgTimestamp}};


#[diesel(table_name = chg_inventory_ms::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: PgNumeric,
    pub created_at: Option<PgTimestamp>,
    pub updated_at: Option<PgTimestamp>,
}