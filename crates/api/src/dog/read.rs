use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::errors::ApiError;

#[derive(Deserialize)]
pub struct DogById {
    id: i32,
}

#[derive(Deserialize)]
pub struct DogByBreedStatus {
    #[serde(with = "db::BreedDef")]
    breed: db::Breed,
    alive: bool,
}

pub async fn all_names(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let names = db::dog::all_names()
        .bind(&tx)
        .all()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(json!({"names": names})))
}

pub async fn dog_by_id(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(DogById { id }): Json<DogById>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let dog = db::dog::dog_by_id()
        .bind(&tx, &id)
        .one()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(json!({"dog": dog})))
}

pub async fn dogs_by_breed_and_status(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(DogByBreedStatus { breed, alive }): Json<DogByBreedStatus>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let dogs = db::dog::dogs_by_breed_and_status()
        .bind(&tx, &breed, &alive)
        .all()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(json!({"dogs": dogs})))
}
