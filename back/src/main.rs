use axum::{
    routing::{get, post},
    serve, Router,
};

mod auth;

#[tokio::main]
async fn main() {
    let api = Router::new()
        .route("/authenticate", post(auth::login))
        .route("/news/create", post(|| async { "hi" }))
        .route("/news/read", get(|| async { "hi" }))
        .route("/news/update", post(|| async { "hi" }))
        .route("/news/delete", post(|| async { "hi" }))
        .route("/dogs/create", post(|| async { "hi" }))
        .route("/dogs/read", get(|| async { "hi" }))
        .route("/dogs/update", post(|| async { "hi" }))
        .route("/dogs/delete", post(|| async { "hi" }))
        .route("/litters/create", post(|| async { "hi" }))
        .route("/litters/read", get(|| async { "hi" }))
        .route("/litters/update", post(|| async { "hi" }))
        .route("/litters/delete", post(|| async { "hi" }))
        .route("/puppies/create", post(|| async { "hi" }))
        .route("/puppies/read", get(|| async { "hi" }))
        .route("/puppies/update", post(|| async { "hi" }))
        .route("/puppies/delete", post(|| async { "hi" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind TCP listener");

    serve(listener, api).await.expect("Failed to start");
}
