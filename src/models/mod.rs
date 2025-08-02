use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QrRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct QrResponse {
    pub qr_code: String,
    pub format: String,
}

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