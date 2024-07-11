use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError};

#[derive(Deserialize)]
pub struct DeleteArticle {
    id: i32,
}

pub async fn delete_article(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(DeleteArticle { id }): Json<DeleteArticle>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    db::news::delete_news()
        .bind(&tx, &id)
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    tx.commit().await.map_err(|_| ApiError::DatabaseError)?;

    Ok(StatusCode::OK)
}
