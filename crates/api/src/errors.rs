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
