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
        .route("/litter/available", get(read::available_litters))
        .route("/litter/one", post(read::litter_by_id))
        .route("/litter/breed", post(read::available_litters_by_breed))
        .route("/litter/new", put(new::new_litter))
        .route("/litter/update", put(update::update_litter))
        .route("/litter/delete", delete(delete::delete_litter))
        .route("/litter/delete/image", delete(delete::delete_image))
        .route(
            "/litter/delete/parents",
            delete(delete::delete_parents_image),
        )
}
