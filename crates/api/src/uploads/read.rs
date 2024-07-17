use axum::{body::Body, extract::Path};
use tokio_util::io::ReaderStream;

use crate::errors::ApiError;

pub async fn read_image(Path(name): Path<String>) -> Result<Body, ApiError> {
    let dir = std::env::var("UPLOAD_DIR")?;
    let path = std::path::Path::new(&dir).join(&name);

    if !path.exists() {
        let msg = format!("Requested image '{}' does not exist", &name);
        return Err(ApiError::NotFound(msg));
    }

    let file = tokio::fs::File::open(path).await?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok(body)
}
