use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::errors::ApiError;

#[derive(Deserialize)]
pub struct NewLitter {
    name: String,
    #[serde(with = "db::BreedDef")]
    breed: db::Breed,
    parents_image: String,
    images: Vec<String>,
}

pub async fn new_litter(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(litter): Json<NewLitter>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let id = db::litter::insert_litter()
        .bind(
            &tx,
            &litter.name,
            &litter.breed,
            &litter.parents_image,
            &litter.images,
        )
        .one()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(json!({"id": id})))
}
