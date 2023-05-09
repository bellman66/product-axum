mod domain;

use std::net::SocketAddr;
use axum::{Router, Server, routing::get};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Check Server Status
    println!("Server Status is Ok");

    let router = Router::new().route("/", get(domain::index::get_status));

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
