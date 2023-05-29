use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use chrono::{NaiveDateTime};
use tokio_postgres::Client;
use crate::domain::rental::Rental;

pub async fn get_rentals(State(client): State<Arc<Client>>) -> Json<Vec<Rental>> {
    let rows = client.query("SELECT * FROM rental", &[]).await.unwrap();

    // FIXME - Enum To Json
    let rentals:Vec<Rental> = rows.iter().map(|row| Rental::from(row)).collect();
    Json(rentals)
}
