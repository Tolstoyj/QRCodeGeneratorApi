use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    QrGeneration(String),
    ImageEncoding(String),
    ValidationError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, error_code) = match self {
            AppError::QrGeneration(msg) => (StatusCode::BAD_REQUEST, msg, "QR_GENERATION_ERROR"),
            AppError::ImageEncoding(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg, "IMAGE_ENCODING_ERROR"),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg, "VALIDATION_ERROR"),
        };

        let body = Json(ErrorResponse {
            error: error_message,
            code: error_code.to_string(),
        });

        (status, body).into_response()
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::QrGeneration(msg) => write!(f, "QR Generation Error: {}", msg),
            AppError::ImageEncoding(msg) => write!(f, "Image Encoding Error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}