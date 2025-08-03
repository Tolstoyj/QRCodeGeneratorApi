pub mod v2;

use serde::Serialize;

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