use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{errors::ApiError, util::de_primitive};

#[derive(Deserialize)]
pub struct DeletePuppy {
    #[serde(deserialize_with = "de_primitive")]
    id: i32,
}

pub async fn delete_puppy(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(DeletePuppy { id }): Json<DeletePuppy>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    db::puppy::delete_puppy()
        .bind(&tx, &id)
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    tx.commit().await.map_err(|_| ApiError::DatabaseError)?;

    Ok(StatusCode::OK)
}
