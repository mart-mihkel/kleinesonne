use axum::{
    routing::{delete, post, put},
    Router,
};

mod delete;
mod new;
mod read;
mod update;

pub fn routes() -> Router {
    Router::new()
        .route("/puppy/names", post(read::names_by_litter))
        .route("/puppy/one", post(read::puppy_by_id))
        .route("/puppy/litter", post(read::puppies_by_litter))
        .route("/puppy/available", post(read::available_puppies_by_litter))
        .route("/puppy/new", put(new::new_puppy))
        .route("/puppy/update", put(update::update_puppy))
        .route("/puppy/delete", delete(delete::delete_puppy))
        .route("/puppy/delete/image", delete(delete::delete_image))
}
