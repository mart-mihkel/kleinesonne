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

pub async fn delete_litter(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<ApiResponse<String>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    db::litter::delete_litter().bind(&tx, &id).await?;
    tx.commit().await?;

    tracing::info!("Delete litter, id = {}", &id);

    Ok(ApiResponse::Success)
}

pub async fn delete_parents_image(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<ApiResponse<String>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    db::litter::delete_litter_parents_image()
        .bind(&tx, &id)
        .await?;

    tx.commit().await?;

    tracing::info!("Delete litter parents image, id = {}", &id);

    Ok(ApiResponse::Success)
}

pub async fn delete_image(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(DeleteImage { id, image }): Json<DeleteImage>,
) -> Result<ApiResponse<String>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    db::litter::delete_litter_image()
        .bind(&tx, &image, &id)
        .await?;

    tx.commit().await?;

    tracing::info!("Delete litter image '{}', id = {}", &image, &id);

    Ok(ApiResponse::Success)
}
