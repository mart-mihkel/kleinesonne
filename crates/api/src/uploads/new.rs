use std::{fs::File, path::Path};

use axum::Json;
use base64::engine::Engine;
use image::{imageops::FilterType, ImageFormat};
use serde::Deserialize;
use tokio::task::JoinSet;

use crate::{auth::jwt::Claims, errors::ApiError, res::ApiResponse};

#[derive(Deserialize)]
pub struct Upload {
    name: String,
    b64: String,
}

pub async fn upload_image(
    _claims: Claims,
    Json(uploads): Json<Vec<Upload>>,
) -> Result<ApiResponse<String>, ApiError> {
    let mut set = JoinSet::new();

    for u in uploads {
        set.spawn_blocking(move || Ok::<ApiResponse<String>, ApiError>(write_image(u)?));
    }

    while let Some(res) = set.join_next().await {
        res??;
    }

    Ok(ApiResponse::Success)
}

fn write_image(Upload { name, b64 }: Upload) -> Result<ApiResponse<String>, ApiError> {
    let dir = std::env::var("UPLOAD_DIR")?;
    let pieces = name.split("/").last();
    let filename = if let Some(last) = pieces { last } else { &name };
    let bytes = base64::engine::general_purpose::STANDARD.decode(b64)?;

    let mut out = File::create_new(Path::new(&dir).join(&filename))?;
    image::load_from_memory(&bytes)?.write_to(&mut out, ImageFormat::Avif)?;

    let mut out = File::create_new(Path::new(&dir).join(format!("sm-{}", &filename)))?;
    image::load_from_memory(&bytes)?
        .resize(400, 400, FilterType::CatmullRom)
        .write_to(&mut out, ImageFormat::Avif)?;

    Ok(ApiResponse::Success)
}
