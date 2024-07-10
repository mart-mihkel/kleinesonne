use std::sync::{Arc, Mutex};

use axum::{serve, Extension, Router};
use db::{admin, Client};
use tokio::net::TcpListener;

mod admin;
mod dog;
mod errors;
mod util;

#[tokio::main]
async fn main() {
    let client = Arc::new(Mutex::new(db::connect().await.unwrap()));

    let routes = Router::new()
        .merge(admin::routes())
        .layer(Extension(client));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    serve(listener, routes).await.unwrap();
}
