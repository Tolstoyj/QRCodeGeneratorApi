pub mod enums;
pub mod colors;
pub mod requests;

pub use enums::{QrSize, ErrorCorrectionLevel, OutputFormat};
pub use colors::QrColors;
pub use requests::{QrCustomization, QrRequest, QrResponse};

use serde::Serialize;

// Health check models
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