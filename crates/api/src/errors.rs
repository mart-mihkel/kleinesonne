use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub enum ApiError {
    Internal,
    NotFound,
    DatabaseError,
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (code, message) = match self {
            ApiError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Intenral server error"),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Requested resource not found"),
            ApiError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            ApiError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            ApiError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            ApiError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            ApiError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };

        let body = json!({"error": message});
        (code, Json(body)).into_response()
    }
}

impl From<std::io::Error> for ApiError {
    fn from(_value: std::io::Error) -> Self {
        ApiError::Internal
    }
}

impl From<tokio::task::JoinError> for ApiError {
    fn from(_value: tokio::task::JoinError) -> Self {
        ApiError::Internal
    }
}

impl From<tokio_postgres::error::Error> for ApiError {
    fn from(_value: tokio_postgres::error::Error) -> Self {
        ApiError::DatabaseError
    }
}

impl From<std::env::VarError> for ApiError {
    fn from(_value: std::env::VarError) -> Self {
        ApiError::Internal
    }
}

impl From<image::ImageError> for ApiError {
    fn from(_value: image::ImageError) -> Self {
        ApiError::Internal
    }
}
