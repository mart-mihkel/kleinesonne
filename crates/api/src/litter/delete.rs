use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError};

#[derive(Deserialize)]
pub struct DeleteLitter {
    id: i32,
}

pub async fn delete_litter(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(DeleteLitter { id }): Json<DeleteLitter>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    db::litter::delete_litter().bind(&tx, &id).await?;
    tx.commit().await?;

    Ok(StatusCode::OK)
}
