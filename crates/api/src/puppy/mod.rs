use axum::{
    routing::{delete, get, post, put},
    Router,
};

mod delete;
mod new;
mod read;
mod update;

pub fn routes() -> Router {
    Router::new()
        .route("/puppy", get(read::puppies_by_litter))
        .route("/puppy", post(new::new_puppy))
        .route("/puppy", put(update::update_puppy))
        .route("/puppy", delete(delete::delete_puppy))
}
