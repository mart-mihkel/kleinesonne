use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub enum ApiResponse<D: Serialize> {
    Success,
    Token(D),
    Data(D),
}

impl<D: Serialize> IntoResponse for ApiResponse<D> {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::Success => {
                let body = Json(json!({"message": "success"}));
                (StatusCode::OK, body).into_response()
            }
            ApiResponse::Token(t) => {
                let body = Json(json!({"token": t}));
                (StatusCode::OK, body).into_response()
            }
            ApiResponse::Data(d) => {
                let body = Json(json!({"data": d}));
                (StatusCode::OK, body).into_response()
            }
        }
    }
}
