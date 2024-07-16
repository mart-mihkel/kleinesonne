use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::errors::ApiError;

#[derive(Deserialize)]
pub struct ById {
    id: i32,
}

#[derive(Deserialize)]
pub struct NewsRead {
    from: i64,
    n: i64,
}

pub async fn all_titles(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let titles = db::news::all_titles().bind(&tx).all().await?;

    Ok(Json(titles))
}

pub async fn article_by_id(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let article = db::news::article_by_id().bind(&tx, &id).one().await?;

    Ok(Json(article))
}

pub async fn n_news_older_than(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(NewsRead { from, n }): Json<NewsRead>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let news = db::news::n_news_older_than()
        .bind(&tx, &from, &n)
        .all()
        .await?;

    Ok(Json(news))
}
