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
pub struct ByLitterId {
    litter_id: i32,
}

pub async fn puppy_by_id(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let puppy = db::puppy::puppy_by_id()
        .bind(&tx, &id)
        .all()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(puppy))
}

pub async fn names_by_litter(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ByLitterId { litter_id }): Json<ByLitterId>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let names = db::puppy::names_by_litter()
        .bind(&tx, &litter_id)
        .all()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(names))
}

pub async fn puppies_by_litter(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ByLitterId { litter_id }): Json<ByLitterId>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let puppies = db::puppy::puppies_by_litter()
        .bind(&tx, &litter_id)
        .all()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(puppies))
}

pub async fn available_puppies_by_litter(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ByLitterId { litter_id }): Json<ByLitterId>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let puppies = db::puppy::avaliable_puppies_by_litter()
        .bind(&tx, &litter_id)
        .all()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(puppies))
}
