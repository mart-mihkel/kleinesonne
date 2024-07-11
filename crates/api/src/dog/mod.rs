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
        .route("/dog/names", get(read::all_names))
        .route("/dog/one", post(read::dog_by_id))
        .route("/dog/breed", post(read::dogs_by_breed_and_status))
        .route("/dog/new", put(new::new_dog))
        .route("/dog/update", put(update::update_dog))
        .route("/dog/delete", delete(delete::delete_dog))
}
