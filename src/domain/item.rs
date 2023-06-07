use tokio_postgres::Row;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RentedItem {
    id: i32,
    rental_id: i32,
    item_id: i32,
    title: String,
}

impl From<&Row> for RentedItem {
    fn from(value: &Row) -> Self {
        Self {
            id: value.get("id"),
            rental_id: value.get("rental_id"),
            item_id: value.get("item_id"),
            title: value.get("title")
        }
    }
}
