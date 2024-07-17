use std::sync::Arc;

use axum::{serve, Extension, Router};
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::cors::CorsLayer;

mod auth;
mod dog;
mod errors;
mod litter;
mod news;
mod puppy;
mod res;
mod uploads;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    tracing::info!("Api started");

    let client = Arc::new(Mutex::new(db::connect().await.unwrap()));

    let routes = Router::new()
        .merge(auth::routes())
        .merge(uploads::routes())
        .merge(dog::routes())
        .merge(litter::routes())
        .merge(puppy::routes())
        .merge(news::routes())
        .layer(Extension(client))
        .layer(CorsLayer::permissive());

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Listening on {}", &addr);

    serve(listener, routes).await.unwrap();
}
