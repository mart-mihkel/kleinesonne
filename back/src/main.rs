use axum::{
    routing::{get, post},
    serve, Router,
};

mod auth;

#[tokio::main]
async fn main() {
    let api = Router::new().route("/admin/login", post(auth::login));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind TCP listener");

    serve(listener, api).await.expect("Failed to start");
}
