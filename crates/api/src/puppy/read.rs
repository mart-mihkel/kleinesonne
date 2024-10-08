use std::sync::Arc;

use axum::{Extension, Json};
use db::puppy::{NamesByLitter, Puppy};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{errors::ApiError, res::ApiResponse};

#[derive(Deserialize)]
pub struct ById {
    id: i32,
}

#[derive(Deserialize)]
pub struct ByLitterId {
    litter_id: i32,
}

pub async fn puppy_by_id(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<ApiResponse<Puppy>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let puppy = db::puppy::puppy_by_id().bind(&tx, &id).one().await?;

    Ok(ApiResponse::Data(puppy))
}

pub async fn names_by_litter(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ByLitterId { litter_id }): Json<ByLitterId>,
) -> Result<ApiResponse<Vec<NamesByLitter>>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let names = db::puppy::names_by_litter()
        .bind(&tx, &litter_id)
        .all()
        .await?;

    Ok(ApiResponse::Data(names))
}

pub async fn puppies_by_litter(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ByLitterId { litter_id }): Json<ByLitterId>,
) -> Result<ApiResponse<Vec<Puppy>>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let puppies = db::puppy::puppies_by_litter()
        .bind(&tx, &litter_id)
        .all()
        .await?;

    Ok(ApiResponse::Data(puppies))
}

pub async fn available_puppies_by_litter(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ByLitterId { litter_id }): Json<ByLitterId>,
) -> Result<ApiResponse<Vec<Puppy>>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let puppies = db::puppy::avaliable_puppies_by_litter()
        .bind(&tx, &litter_id)
        .all()
        .await?;

    Ok(ApiResponse::Data(puppies))
}
