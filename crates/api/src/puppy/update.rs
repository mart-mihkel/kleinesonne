use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError};

#[derive(Deserialize)]
pub struct UpdatePuppy {
    id: i32,
    litter_id: i32,
    name: String,
    #[serde(with = "db::GenderDef")]
    gender: db::Gender,
    #[serde(with = "db::AvailabilityDef")]
    availability: db::Availability,
    image: Option<String>,
}

pub async fn update_puppy(
    _claims: Claims,
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
