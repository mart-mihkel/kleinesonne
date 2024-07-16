use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::errors::ApiError;

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
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let names = db::litter::all_names().bind(&tx).all().await?;

    Ok(Json(names))
}

pub async fn litter_by_id(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let litter = db::litter::litter_by_id().bind(&tx, &id).one().await?;

    Ok(Json(litter))
}

pub async fn available_litters(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let litter = db::litter::available_litters().bind(&tx).all().await?;

    Ok(Json(litter))
}

pub async fn available_litters_by_breed(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ByBreed { breed }): Json<ByBreed>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let litter = db::litter::available_litters_by_breed()
        .bind(&tx, &breed)
        .all()
        .await?;

    Ok(Json(litter))
}
