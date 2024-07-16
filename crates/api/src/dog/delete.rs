use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError, res::ApiResponse};

#[derive(Deserialize)]
pub struct ById {
    id: i32,
}

#[derive(Deserialize)]
pub struct DeleteImage {
    id: i32,
    image: String,
}

pub async fn delete_dog(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<ApiResponse<String>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    db::dog::delete_dog().bind(&tx, &id).await?;
    tx.commit().await?;

    Ok(ApiResponse::Success)
}

pub async fn delete_thumbnail(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<ApiResponse<String>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    db::dog::delete_dog_thumbnail().bind(&tx, &id).await?;
    tx.commit().await?;

    Ok(ApiResponse::Success)
}

pub async fn delete_image(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(DeleteImage { id, image }): Json<DeleteImage>,
) -> Result<ApiResponse<String>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    db::dog::delete_dog_image().bind(&tx, &image, &id).await?;
    tx.commit().await?;

    Ok(ApiResponse::Success)
}
