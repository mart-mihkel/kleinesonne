use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError};

#[derive(Deserialize)]
pub struct ById {
    id: i32,
}

#[derive(Deserialize)]
pub struct DeleteImage {
    id: i32,
    image: String,
}

pub async fn delete_article(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    db::news::delete_news().bind(&tx, &id).await?;
    tx.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn delete_image(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(DeleteImage { id, image }): Json<DeleteImage>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    db::news::delete_article_image()
        .bind(&tx, &image, &id)
        .await?;

    tx.commit().await?;

    Ok(StatusCode::OK)
}
