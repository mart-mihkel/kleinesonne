#[tokio::main]
async fn main() {
    let api = axum::Router::new().route("/", axum::routing::get(|| async { "Lest!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, api).await.unwrap();
}
