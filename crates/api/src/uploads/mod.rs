use axum::{
    routing::{get, post},
    Router,
};

mod new;
mod read;

pub fn routes() -> Router {
    Router::new()
        .route("/uploads/:name", get(read::read_image))
        .route("/uploads", post(new::upload_image))
}
