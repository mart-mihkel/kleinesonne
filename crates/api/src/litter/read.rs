use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::errors::ApiError;

#[derive(Deserialize)]
pub struct LitterRead {
    id: i32,
}

pub async fn all_names(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let names = db::litter::all_names()
        .bind(&tx)
        .all()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(json!({"names": names})))
}

pub async fn litter_by_id(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(LitterRead { id }): Json<LitterRead>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let litter = db::litter::litter_by_id()
        .bind(&tx, &id)
        .all()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(json!({"litter": litter})))
}
