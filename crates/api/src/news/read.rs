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
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let titles = db::news::all_titles()
        .bind(&tx)
        .all()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(titles))
}

pub async fn article_by_id(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let article = db::news::article_by_id()
        .bind(&tx, &id)
        .one()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(article))
}

pub async fn n_news_older_than(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(NewsRead { from, n }): Json<NewsRead>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let news = db::news::n_news_older_than()
        .bind(&tx, &from, &n)
        .all()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(news))
}
