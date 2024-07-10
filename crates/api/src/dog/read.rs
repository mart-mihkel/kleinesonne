use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::{errors::ApiError, util::de_primitive};

#[derive(Deserialize)]
pub struct DogById {
    #[serde(deserialize_with = "de_primitive")]
    id: i32,
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

    Ok(Json(json!({"data": dog})))
}

#[derive(Deserialize)]
pub struct DogByBreedStatus {
    #[serde(with = "db::BreedDef")]
    breed: db::Breed,
    #[serde(deserialize_with = "de_primitive")]
    alive: bool,
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

    Ok(Json(json!({"data": dogs})))
}
