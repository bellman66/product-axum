
use chrono;
use chrono::{NaiveDateTime};
use tokio_postgres::Row;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RentedItem {
    id: i32,
    book_id: i32,
    rental_id: i32,
    rented_date: NaiveDateTime,
    due_date: NaiveDateTime,
    book_title: String
}

impl From<&Row> for RentedItem {
    fn from(value: &Row) -> Self {
        Self {
            id: value.get("id"),
            book_id: value.get("book_id"),
            rental_id: value.get("rental_id"),
            rented_date: value.get("rented_date"),
            due_date: value.get("due_date"),
            book_title: value.get("book_title")
        }
    }
}
