use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError, res::ApiResponse};

#[derive(Deserialize)]
pub struct UpdateDog {
    id: i32,
    name: String,
    nickname: String,
    father: String,
    mother: String,
    dob: i64,
    #[serde(with = "db::BreedDef")]
    breed: db::Breed,
    #[serde(with = "db::GenderDef")]
    gender: db::Gender,
    alive: bool,
    thumbnail: Option<String>,
    health: Vec<String>,
    titles: Vec<String>,
    images: Vec<String>,
}

pub async fn update_dog(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(dog): Json<UpdateDog>,
) -> Result<ApiResponse<String>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    db::dog::update_dog()
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
            &dog.id,
        )
        .await?;

    tx.commit().await?;

    tracing::info!("Update dog , id = {}", &dog.id);

    Ok(ApiResponse::Success)
}
