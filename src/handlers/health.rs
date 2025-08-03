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
                method: "POST".to_string(),
                description: "Generate customized QR code as JSON with base64 image".to_string(),
            },
            EndpointInfo {
                path: "/generate".to_string(),
                method: "GET".to_string(),
                description: "Generate customized QR code via query parameters".to_string(),
            },
            EndpointInfo {
                path: "/image".to_string(),
                method: "POST".to_string(),
                description: "Generate customized QR code as downloadable image".to_string(),
            },
        ],
    })
}