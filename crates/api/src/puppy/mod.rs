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
        .route("/puppy/litter", post(read::puppies_by_litter))
        .route("/puppy/new", put(new::new_puppy))
        .route("/puppy/modify", put(update::update_puppy))
        .route("/puppy/delete", delete(delete::delete_puppy))
}
