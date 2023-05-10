mod domain;
mod web;

use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Router, Server, routing::get};
use axum::routing::Route;
use tokio_postgres::{Client, Config, NoTls};

#[tokio::main]
async fn main() {
    // Setting Database
    let (client, connection) = Config::new().host("localhost").user("testuser").port(5432).password("testuser").dbname("product").connect(NoTls).await.unwrap();

    // Connection Spawn
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error : {}", e);
        }
    });

    let shared_client: Arc<Client> = Arc::new(client);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Router Setting
    let rental_router = Router::new()
        .route("/", get(web::controller::rental_controller::get_rentals))
        .with_state(shared_client.clone());

    let item_router = Router::new()
        .route("/", get(web::controller::item_controller::get_rented_items))
        .with_state(shared_client.clone());

    let router = Router::new()
        .route("/", get(web::controller::index::get_status))
        .nest("/item", item_router)
        .nest("/rental", rental_router);

    // Check Server Status
    println!("Server Status is Ok");

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
