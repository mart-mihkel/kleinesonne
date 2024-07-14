use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError};

#[derive(Deserialize)]
pub struct NewArticle {
    title: String,
    text: String,
    date: i64,
    images: Vec<String>,
}

pub async fn new_article(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(article): Json<NewArticle>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let id = db::news::insert_news()
        .bind(
            &tx,
            &article.title,
            &article.text,
            &article.date,
            &article.images,
        )
        .one()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    tx.commit().await.map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(json!({"id": id})))
}
