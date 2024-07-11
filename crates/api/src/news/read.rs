use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::{errors::ApiError, util::de_primitive};

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

    Ok(Json(json!({"titles": titles})))
}

#[derive(Deserialize)]
pub struct NewsRead {
    #[serde(deserialize_with = "de_primitive")]
    from: i64,
    #[serde(deserialize_with = "de_primitive")]
    n: i64,
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

    Ok(Json(json!({"news": news})))
}
