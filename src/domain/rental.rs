use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize, ToSql, FromSql)]
pub enum RentalStatus{
    RentAvailable ,
    RentUnavailable
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rental {
    id: i32,
    user_id: i32,
    rental_status: RentalStatus
}

impl From<&Row> for Rental {
    fn from(value: &Row) -> Self {
        Self {
            id: value.get("id"),
            user_id: value.get("user_id"),
            rental_status: value.get("rental_status")
        }
    }
}
