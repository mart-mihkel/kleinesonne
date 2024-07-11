use std::sync::Arc;

use axum::{serve, Extension, Router};
use tokio::{net::TcpListener, sync::Mutex};

mod admin;
mod dog;
mod errors;
mod news;
mod util;

#[tokio::main]
async fn main() {
    let client = Arc::new(Mutex::new(db::connect().await.unwrap()));

    let routes = Router::new()
        .merge(admin::routes())
        .merge(dog::routes())
        .merge(news::routes())
        .layer(Extension(client));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    serve(listener, routes).await.unwrap();
}
