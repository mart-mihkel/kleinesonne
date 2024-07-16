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
pub struct ByBreedGender {
    #[serde(with = "db::BreedDef")]
    breed: db::Breed,
    #[serde(with = "db::GenderDef")]
    gender: db::Gender,
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
    let names = db::dog::all_names().bind(&tx).all().await?;

    Ok(Json(names))
}

pub async fn dog_by_id(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let dog = db::dog::dog_by_id().bind(&tx, &id).one().await?;

    Ok(Json(dog))
}

pub async fn alive_dogs_by_breed_and_gender(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ByBreedGender { breed, gender }): Json<ByBreedGender>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let dogs = db::dog::alive_dogs_by_breed_and_gender()
        .bind(&tx, &breed, &gender)
        .all()
        .await?;

    Ok(Json(dogs))
}

pub async fn retired_dogs_by_breed(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ByBreed { breed }): Json<ByBreed>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let dogs = db::dog::retired_dogs_by_breed()
        .bind(&tx, &breed)
        .all()
        .await?;

    Ok(Json(dogs))
}
