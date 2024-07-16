use std::{fs::File, path::Path};

use axum::{http::StatusCode, response::IntoResponse, Json};
use base64::engine::Engine;
use image::{imageops::FilterType, ImageFormat};
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
    let dir = std::env::var("UPLOAD_DIR")?;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|_| ApiError::Internal)?;

    let img = image::load_from_memory(&bytes)?;
    let img_s = image::load_from_memory(&bytes)?;

    let pieces = name.split("/").last();
    let filename = if let Some(last) = pieces { last } else { &name };

    let path = Path::new(&dir).join(&filename);
    let path_s = Path::new(&dir).join(format!("sm-{}", &filename));

    let handle = tokio::task::spawn_blocking(move || {
        let mut out = File::create_new(path)?;
        img.write_to(&mut out, ImageFormat::Avif)?;

        Ok::<(), ApiError>(())
    });

    let handle_s = tokio::task::spawn_blocking(move || {
        let mut out = File::create_new(path_s)?;
        img_s
            .resize(600, 600, FilterType::Triangle)
            .write_to(&mut out, ImageFormat::Avif)?;

        Ok::<(), ApiError>(())
    });

    handle.await??;
    handle_s.await??;

    Ok(())
}
