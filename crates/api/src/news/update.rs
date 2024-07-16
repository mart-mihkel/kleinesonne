use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError, res::ApiResponse};

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
) -> Result<ApiResponse<u64>, ApiError> {
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

    Ok(ApiResponse::Data(id))
}
