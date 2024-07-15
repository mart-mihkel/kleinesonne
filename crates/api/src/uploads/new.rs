use axum::{http::StatusCode, response::IntoResponse, Json};
use base64::engine::Engine;
use serde::Deserialize;

use crate::{auth::jwt::Claims, errors::ApiError};

#[derive(Deserialize)]
pub struct Upload {
    name: String,
    b64: String,
}

pub async fn upload_image(
    _claims: Claims,
    Json(uploads): Json<Vec<Upload>>,
) -> Result<impl IntoResponse, ApiError> {
    for upload in uploads {
        write_image(upload).await?;
    }

    Ok(StatusCode::OK)
}

async fn write_image(Upload { name, b64 }: Upload) -> Result<(), ApiError> {
    let dir = std::env::var("UPLOAD_DIR").map_err(|_| ApiError::Internal)?;
    let path = std::path::Path::new(&dir).join(&name);
    if path.exists() {
        return Err(ApiError::FileExists);
    }

    let image = base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|_| ApiError::Internal)?;

    tokio::fs::write(path, image)
        .await
        .map_err(|_| ApiError::Internal)?;

    Ok(())
}
