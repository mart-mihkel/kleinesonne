use std::{
    cell::Cell,
    sync::{Arc, Mutex},
};

use axum::{serve, Extension, Router};
use db::Client;
use tokio::net::TcpListener;

mod admin;
mod errors;

#[tokio::main]
async fn main() {
    let client = Arc::new(Cell::new(db::connect().await.unwrap()));

    let routes = Router::new().layer(Extension(client));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    serve(listener, routes).await.unwrap();
}
