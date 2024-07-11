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
        .route("/litter", get(read::litter_by_id))
        .route("/litter", post(new::new_litter))
        .route("/litter", put(update::update_litter))
        .route("/litter", delete(delete::delete_litter))
}
