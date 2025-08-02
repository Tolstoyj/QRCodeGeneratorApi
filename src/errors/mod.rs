use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    GenerationError(String),
    ValidationError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message, error_code) = match self {
            ApiError::GenerationError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg, "GENERATION_ERROR"),
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg, "VALIDATION_ERROR"),
        };

        let body = Json(ErrorResponse {
            error: error_message,
            code: error_code.to_string(),
        });

        (status, body).into_response()
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::GenerationError(msg) => write!(f, "Generation Error: {}", msg),
            ApiError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}