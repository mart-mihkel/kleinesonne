use axum::{routing::post, Router};

mod new;
mod read;

pub fn routes() -> Router {
    Router::new()
        .route("/dog/id", post(read::dog_by_id))
        .route("/dogs/breed", post(read::dogs_by_breed_and_status))
        .route("/dogs/new", post(new::new_dog))
}
