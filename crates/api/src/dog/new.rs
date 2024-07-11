use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError, util::de_primitive};

#[derive(Deserialize)]
pub struct NewDog {
    name: String,
    nickname: String,
    father: String,
    mother: String,
    #[serde(deserialize_with = "de_primitive")]
    dob: i64,
    #[serde(with = "db::BreedDef")]
    breed: db::Breed,
    #[serde(with = "db::GenderDef")]
    gender: db::Gender,
    #[serde(deserialize_with = "de_primitive")]
    alive: bool,
    thumbnail: String,
    health: Vec<String>,
    titles: Vec<String>,
    images: Vec<String>,
}

pub async fn new_dog(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(dog): Json<NewDog>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let id = db::dog::insert_dog()
        .bind(
            &tx,
            &dog.name,
            &dog.nickname,
            &dog.father,
            &dog.mother,
            &dog.breed,
            &dog.gender,
            &dog.dob,
            &dog.alive,
            &dog.thumbnail,
            &dog.images,
            &dog.health,
            &dog.titles,
        )
        .one()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(json!({"id": id})))
}
