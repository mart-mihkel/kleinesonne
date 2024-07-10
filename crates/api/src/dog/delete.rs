use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::errors::ApiError;

#[derive(Deserialize)]
pub struct DeleteDog {
    id: i32,
}

pub async fn delete_dog(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(DeleteDog { id }): Json<DeleteDog>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let id = db::dog::delete_dog()
        .bind(&tx, &id)
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(json!({"id": id})))
}
