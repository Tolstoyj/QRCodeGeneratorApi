use axum::Json;

use crate::models::{HealthResponse, EndpointInfo};

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        endpoints: vec![
            // V1 Endpoints (Legacy)
            EndpointInfo {
                path: "/".to_string(),
                method: "GET".to_string(),
                description: "Health check endpoint".to_string(),
            },
            EndpointInfo {
                path: "/generate".to_string(),
                method: "GET".to_string(),
                description: "V1: Generate QR code as JSON with base64 image".to_string(),
            },
            EndpointInfo {
                path: "/image".to_string(),
                method: "GET".to_string(),
                description: "V1: Generate QR code as downloadable PNG image".to_string(),
            },
            // V2 Endpoints (Enhanced)
            EndpointInfo {
                path: "/v2/generate".to_string(),
                method: "POST".to_string(),
                description: "V2: Generate customized QR code as JSON with base64 image".to_string(),
            },
            EndpointInfo {
                path: "/v2/generate".to_string(),
                method: "GET".to_string(),
                description: "V2: Generate customized QR code via query parameters".to_string(),
            },
            EndpointInfo {
                path: "/v2/image".to_string(),
                method: "POST".to_string(),
                description: "V2: Generate customized QR code as downloadable image".to_string(),
            },
        ],
    })
}