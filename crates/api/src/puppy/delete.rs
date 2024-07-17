use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError, res::ApiResponse};

#[derive(Deserialize)]
pub struct ById {
    id: i32,
}

pub async fn delete_puppy(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<ApiResponse<String>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    db::puppy::delete_puppy().bind(&tx, &id).await?;
    tx.commit().await?;

    tracing::info!("Delete puppy, id = {}", &id);

    Ok(ApiResponse::Success)
}

pub async fn delete_image(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<ApiResponse<String>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    db::puppy::delete_puppy_image().bind(&tx, &id).await?;
    tx.commit().await?;

    tracing::info!("Delete puppy image, id = {}", &id);

    Ok(ApiResponse::Success)
}
