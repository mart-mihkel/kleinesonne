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
        .route("/news/titles", get(read::all_titles))
        .route("/news/one", post(read::article_by_id))
        .route("/news/from", post(read::n_news_older_than))
        .route("/news/new", put(new::new_article))
        .route("/news/update", put(update::new_article))
        .route("/news/delete", delete(delete::delete_article))
}
