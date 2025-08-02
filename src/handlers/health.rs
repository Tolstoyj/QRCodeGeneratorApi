use axum::Json;

use crate::models::{HealthResponse, EndpointInfo};

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        endpoints: vec![
            EndpointInfo {
                path: "/".to_string(),
                method: "GET".to_string(),
                description: "Health check endpoint".to_string(),
            },
            EndpointInfo {
                path: "/generate".to_string(),
                method: "GET".to_string(),
                description: "Generate QR code as JSON with base64 image".to_string(),
            },
            EndpointInfo {
                path: "/image".to_string(),
                method: "GET".to_string(),
                description: "Generate QR code as downloadable PNG image".to_string(),
            },
        ],
    })
}