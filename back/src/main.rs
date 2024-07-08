use axum::{
    routing::{get, post},
    serve, Router,
};
use tokio_rusqlite::Connection;

mod auth;
mod dogs;
mod news;

#[tokio::main]
async fn main() {
    let conn = Connection::open("/tmp/test.db")
        .await
        .expect("Failed to connect to database");

    let api = Router::new()
        .route("/login", post(auth::login))
        .route("/news/create", post(news::create))
        .route("/news/read", get(news::read))
        .route("/news/update", post(news::update))
        .route("/news/delete", post(news::delete))
        .route("/dogs/create", post(dogs::create))
        .route("/dogs/read", get(dogs::read))
        .route("/dogs/update", post(dogs::update))
        .route("/dogs/delete", post(dogs::delete))
        .route("/litters/create", post(|| async { "hi" }))
        .route("/litters/read", get(|| async { "hi" }))
        .route("/litters/update", post(|| async { "hi" }))
        .route("/litters/delete", post(|| async { "hi" }))
        .route("/puppies/create", post(|| async { "hi" }))
        .route("/puppies/read", get(|| async { "hi" }))
        .route("/puppies/update", post(|| async { "hi" }))
        .route("/puppies/delete", post(|| async { "hi" }))
        .with_state(conn);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind TCP listener");

    serve(listener, api).await.expect("Failed to start");
}
