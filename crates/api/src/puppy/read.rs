use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::{errors::ApiError, util::de_primitive};

#[derive(Deserialize)]
pub struct ById {
    #[serde(deserialize_with = "de_primitive")]
    litter_id: i32,
}

pub async fn names_by_litter(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { litter_id }): Json<ById>,
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

    Ok(Json(json!({"names": names})))
}

pub async fn puppies_by_litter(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { litter_id }): Json<ById>,
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

    Ok(Json(json!({"puppies": puppies})))
}
