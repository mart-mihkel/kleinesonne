use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError, res::ApiResponse};

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
) -> Result<ApiResponse<i32>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let id = db::news::insert_news()
        .bind(
            &tx,
            &article.title,
            &article.text,
            &article.date,
            &article.images,
        )
        .one()
        .await?;

    tx.commit().await?;

    tracing::info!("Create article, id = {}", &id);

    Ok(ApiResponse::Data(id))
}
