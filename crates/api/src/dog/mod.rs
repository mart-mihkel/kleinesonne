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
        .route("/dog/alive", post(read::alive_dogs_by_breed_and_gender))
        .route("/dog/retired", post(read::retired_dogs_by_breed))
        .route("/dog/new", put(new::new_dog))
        .route("/dog/update", put(update::update_dog))
        .route("/dog/delete", delete(delete::delete_dog))
}
