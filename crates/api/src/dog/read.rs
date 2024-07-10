use std::{
    fmt::Display,
    str::FromStr,
    sync::{Arc, Mutex},
};

use axum::{response::IntoResponse, Extension, Json};
use serde::{Deserialize, Deserializer};

use crate::{errors::ApiError, util::de_primitive};

#[derive(Deserialize)]
struct DogById {
    #[serde(deserialize_with = "de_primitive")]
    id: u64,
}

pub async fn dog_by_id(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(DogById { id }): Json<DogById>,
) -> Result<impl IntoResponse, ApiError> {
    let client = client.lock().map_err(|_| ApiError::Internal)?;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let dog = db::dog::dog_by_id()
        .bind(&tx, &id)
        .one()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(dog)
}

#[derive(Deserialize)]
struct DogByBreedStatus {
    breed: String,
    #[serde(deserialize_with = "de_primitive")]
    alive: bool,
}

pub async fn dogs_by_breed_and_status(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(DogByBreedStatus { breed, alive }): Json(DogByBreedStatus),
) -> Result<impl IntoResponse, ApiError> {
    let client = client.lock().map_err(|_| ApiError::Internal)?;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let dogs = db::dog::dogs_by_breed_and_status()
        .bind(&tx, &breed, &alive)
        .all()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(dogs)
}
