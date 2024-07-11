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
        .route("/news", get(read::n_news_older_than))
        .route("/news", post(new::new_article))
        .route("/news", put(update::new_article))
        .route("/news", delete(delete::delete_article))
}
