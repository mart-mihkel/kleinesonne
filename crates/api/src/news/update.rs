use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError};

#[derive(Deserialize)]
pub struct UpdateArticle {
    id: i32,
    title: String,
    text: String,
    date: i64,
    images: Vec<String>,
}

pub async fn new_article(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(article): Json<UpdateArticle>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let id = db::news::update_news()
        .bind(
            &tx,
            &article.title,
            &article.text,
            &article.date,
            &article.images,
            &article.id,
        )
        .await?;

    tx.commit().await?;

    Ok(Json(json!({"id": id})))
}
