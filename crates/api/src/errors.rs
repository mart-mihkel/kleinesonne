use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub enum ApiError {
    Internal(String),
    NotFound(String),
    DatabaseError(String),
    InvalidToken,
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (code, message) = match self {
            ApiError::Internal(cause) => (StatusCode::INTERNAL_SERVER_ERROR, cause),
            ApiError::NotFound(cause) => (StatusCode::NOT_FOUND, cause),
            ApiError::DatabaseError(cause) => (StatusCode::INTERNAL_SERVER_ERROR, cause),
            ApiError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token".to_string()),
            ApiError::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, "Wrong credentials".to_string())
            }
            ApiError::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "Missing credentials".to_string())
            }
            ApiError::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Token creation error".to_string(),
            ),
        };

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
        ApiError::DatabaseError(value.to_string())
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
