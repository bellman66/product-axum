use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::{NaiveDateTime};
use tokio_postgres::Client;
use crate::domain::item::RentedItem;

pub async fn get_rented_items(State(client): State<Arc<Client>>) -> Json<Vec<RentedItem>> {
    let rows = client.query("SELECT * FROM rented_item", &[]).await.unwrap();
    let rented_items:Vec<RentedItem> = rows.iter().map(|row| RentedItem::from(row)).collect();
    Json(rented_items)
}
