pub mod v2;

use serde::{Deserialize, Serialize};

// V1 Models (Legacy)
#[derive(Deserialize)]
pub struct QrRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct QrResponse {
    pub qr_code: String,
    pub format: String,
}

// Common Models
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub endpoints: Vec<EndpointInfo>,
}

#[derive(Serialize)]
pub struct EndpointInfo {
    pub path: String,
    pub method: String,
    pub description: String,
}