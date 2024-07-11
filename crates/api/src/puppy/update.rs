use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{errors::ApiError, util::de_primitive};

#[derive(Deserialize)]
pub struct UpdatePuppy {
    #[serde(deserialize_with = "de_primitive")]
    id: i32,
    #[serde(deserialize_with = "de_primitive")]
    litter_id: i32,
    name: String,
    #[serde(with = "db::GenderDef")]
    gender: db::Gender,
    #[serde(with = "db::AvailabilityDef")]
    availability: db::Availability,
    image: String,
}

pub async fn update_puppy(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(puppy): Json<UpdatePuppy>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    db::puppy::update_puppy()
        .bind(
            &tx,
            &puppy.litter_id,
            &puppy.name,
            &puppy.gender,
            &puppy.availability,
            &puppy.image,
            &puppy.id,
        )
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    tx.commit().await.map_err(|_| ApiError::DatabaseError)?;

    Ok(StatusCode::OK)
}
