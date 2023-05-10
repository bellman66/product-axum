use std::sync::Arc;
use axum::extract::State;
use chrono::{DateTime, NaiveDateTime, Utc};
use tokio_postgres::Client;

pub async fn get_status() -> &'static str {
    "Server Status Ok "
}
