use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::{errors::ApiError, util::de_primitive};

#[derive(Deserialize)]
pub struct LitterRead {
    #[serde(deserialize_with = "de_primitive")]
    id: i32,
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

    let litter = db::litter::get_litter()
        .bind(&tx, &id)
        .all()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    Ok(Json(json!({"litter": litter})))
}
