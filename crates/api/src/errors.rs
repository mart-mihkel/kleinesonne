use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub enum ApiError {
    NotFound(String),
    Internal(String),
    Database(String),
    Authentication(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (code, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg),
        };

        tracing::error!("status = {}, error = {}", code, message);

        (code, Json(json!({"error": message}))).into_response()
    }
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        ApiError::Internal(value.to_string())
    }
}

impl From<tokio::task::JoinError> for ApiError {
    fn from(value: tokio::task::JoinError) -> Self {
        ApiError::Internal(value.to_string())
    }
}

impl From<tokio_postgres::error::Error> for ApiError {
    fn from(value: tokio_postgres::error::Error) -> Self {
        ApiError::Database(value.to_string())
    }
}

impl From<std::env::VarError> for ApiError {
    fn from(value: std::env::VarError) -> Self {
        ApiError::Internal(value.to_string())
    }
}

impl From<image::ImageError> for ApiError {
    fn from(value: image::ImageError) -> Self {
        ApiError::Internal(value.to_string())
    }
}

impl From<base64::DecodeError> for ApiError {
    fn from(value: base64::DecodeError) -> Self {
        ApiError::Internal(value.to_string())
    }
}

impl From<axum_extra::typed_header::TypedHeaderRejection> for ApiError {
    fn from(value: axum_extra::typed_header::TypedHeaderRejection) -> Self {
        ApiError::Authentication(value.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        ApiError::Authentication(value.to_string())
    }
}
