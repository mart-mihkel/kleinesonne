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
        .route("/litter/names", get(read::all_names))
        .route("/litter/one", post(read::litter_by_id))
        .route("/litter/new", put(new::new_litter))
        .route("/litter/update", put(update::update_litter))
        .route("/litter/delete", delete(delete::delete_litter))
}
