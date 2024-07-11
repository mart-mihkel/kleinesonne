use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{errors::ApiError, util::de_primitive};

#[derive(Deserialize)]
pub struct DeleteLitter {
    #[serde(deserialize_with = "de_primitive")]
    id: i32,
}

pub async fn delete_litter(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(DeleteLitter { id }): Json<DeleteLitter>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    db::litter::delete_litter()
        .bind(&tx, &id)
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    tx.commit().await.map_err(|_| ApiError::DatabaseError)?;

    Ok(StatusCode::OK)
}
