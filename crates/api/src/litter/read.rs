use std::sync::Arc;

use axum::{Extension, Json};
use db::litter::{AllNames, Litter};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{errors::ApiError, res::ApiResponse};

#[derive(Deserialize)]
pub struct ById {
    id: i32,
}

#[derive(Deserialize)]
pub struct ByBreed {
    #[serde(with = "db::BreedDef")]
    breed: db::Breed,
}

pub async fn all_names(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
) -> Result<ApiResponse<Vec<AllNames>>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let names = db::litter::all_names().bind(&tx).all().await?;

    Ok(ApiResponse::Data(names))
}

pub async fn litter_by_id(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<ApiResponse<Litter>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let litter = db::litter::litter_by_id().bind(&tx, &id).one().await?;

    Ok(ApiResponse::Data(litter))
}

pub async fn available_litters(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
) -> Result<ApiResponse<Vec<Litter>>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let litters = db::litter::available_litters().bind(&tx).all().await?;

    Ok(ApiResponse::Data(litters))
}

pub async fn available_litters_by_breed(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ByBreed { breed }): Json<ByBreed>,
) -> Result<ApiResponse<Vec<Litter>>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let litters = db::litter::available_litters_by_breed()
        .bind(&tx, &breed)
        .all()
        .await?;

    Ok(ApiResponse::Data(litters))
}
