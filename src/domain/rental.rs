use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize, ToSql, FromSql)]
pub enum RentalStatus{
    Renting ,
    Return
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rental {
    id: i32,
    user_id: i32,
    rental_status: RentalStatus,
    late_fee: i32,
    rented_date: NaiveDateTime,
    due_date: NaiveDateTime,
}

impl From<&Row> for Rental {
    fn from(value: &Row) -> Self {
        Self {
            id: value.get("id"),
            user_id: value.get("user_id"),
            rental_status: value.get("rental_status"),
            late_fee: value.get("late_fee"),
            rented_date: value.get("rented_date"),
            due_date: value.get("due_date"),
        }
    }
}
