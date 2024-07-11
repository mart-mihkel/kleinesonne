use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError, util::de_primitive};

#[derive(Deserialize)]
pub struct UpdateArticle {
    #[serde(deserialize_with = "de_primitive")]
    id: i32,
    title: String,
    text: String,
    #[serde(deserialize_with = "de_primitive")]
    date: i64,
    images: Vec<String>,
}

pub async fn new_article(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(article): Json<UpdateArticle>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let id = db::news::update_news()
        .bind(
            &tx,
            &article.title,
            &article.text,
            &article.date,
            &article.images,
            &article.id,
        )
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    tx.commit().await.map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(json!({"id": id})))
}
