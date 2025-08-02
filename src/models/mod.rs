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

impl QrRequest {
    pub fn validate(&self) -> Result<(), crate::errors::AppError> {
        if self.url.is_empty() {
            return Err(crate::errors::AppError::ValidationError(
                "URL cannot be empty".to_string(),
            ));
        }

        if self.url.len() > 2048 {
            return Err(crate::errors::AppError::ValidationError(
                "URL too long (max 2048 characters)".to_string(),
            ));
        }

        Ok(())
    }
}