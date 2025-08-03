use serde::{Deserialize, Serialize};
use crate::errors::ApiError;
use super::{QrColors, QrSize, ErrorCorrectionLevel, OutputFormat};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCustomization {
    #[serde(default)]
    pub size: QrSize,
    
    #[serde(default)]
    pub error_correction: ErrorCorrectionLevel,
    
    #[serde(default)]
    pub colors: QrColors,
    
    #[serde(default = "default_border_width")]
    pub border_width: u32,
    
    #[serde(default)]
    pub format: OutputFormat,
}

impl Default for QrCustomization {
    fn default() -> Self {
        Self {
            size: QrSize::default(),
            error_correction: ErrorCorrectionLevel::default(),
            colors: QrColors::default(),
            border_width: default_border_width(),
            format: OutputFormat::default(),
        }
    }
}

impl QrCustomization {
    pub fn validate(&self) -> Result<(), ApiError> {
        // Validate size
        self.size.validate()
            .map_err(|e| ApiError::ValidationError(format!("Size validation failed: {}", e)))?;

        // Validate colors
        self.colors.validate()
            .map_err(|e| ApiError::ValidationError(format!("Color validation failed: {}", e)))?;

        // Validate border width
        if self.border_width > 50 {
            return Err(ApiError::ValidationError(
                "Border width cannot exceed 50 pixels".to_string()
            ));
        }

        // Check contrast for accessibility
        if !self.colors.has_sufficient_contrast()
            .map_err(|e| ApiError::ValidationError(format!("Contrast check failed: {}", e)))? {
            return Err(ApiError::ValidationError(
                "Insufficient color contrast. Please ensure at least 3:1 contrast ratio for accessibility".to_string()
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QrRequest {
    pub url: String,
    
    #[serde(default)]
    pub customization: QrCustomization,
}

impl QrRequest {
    pub fn validate(&self, max_url_length: usize) -> Result<(), ApiError> {
        // Validate URL
        if self.url.trim().is_empty() {
            return Err(ApiError::ValidationError("URL cannot be empty".to_string()));
        }

        if self.url.len() > max_url_length {
            return Err(ApiError::ValidationError(format!(
                "URL too long (max {} characters)", max_url_length
            )));
        }

        // Advanced URL validation
        self.validate_url_format()?;

        // Validate customization
        self.customization.validate()?;

        Ok(())
    }

    fn validate_url_format(&self) -> Result<(), ApiError> {
        let url = self.url.trim();
        
        // Check for basic URL format
        if !url.contains("://") && !url.starts_with("http") && !url.starts_with("mailto:") {
            // Allow plain text for QR codes, but validate it's reasonable
            if url.contains('\n') || url.contains('\r') {
                return Err(ApiError::ValidationError(
                    "Text content cannot contain newlines".to_string()
                ));
            }
            return Ok(()); // Allow plain text
        }

        // For URL-like content, do basic validation
        if url.starts_with("http://") || url.starts_with("https://") {
            // Check for malicious patterns
            let suspicious_patterns = [
                "javascript:", "data:", "vbscript:", "file:", "ftp:",
            ];
            
            let lower_url = url.to_lowercase();
            for pattern in &suspicious_patterns {
                if lower_url.contains(pattern) {
                    return Err(ApiError::ValidationError(
                        format!("URL contains potentially unsafe protocol: {}", pattern)
                    ));
                }
            }

            // Basic URL structure validation
            if !url.contains('.') {
                return Err(ApiError::ValidationError(
                    "URL appears to be malformed (missing domain)".to_string()
                ));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct QrResponse {
    pub qr_code: String,
    pub format: String,
    pub size: String,
    pub error_correction: String,
    pub colors: QrColors,
    pub border_width: u32,
}

impl QrResponse {
    pub fn new(
        qr_code: String,
        customization: &QrCustomization,
    ) -> Self {
        Self {
            qr_code,
            format: customization.format.to_string().to_lowercase(),
            size: customization.size.to_string(),
            error_correction: format!("{:?}", customization.error_correction),
            colors: customization.colors.clone(),
            border_width: customization.border_width,
        }
    }
}

fn default_border_width() -> u32 {
    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_request_validation_success() {
        let request = QrRequest {
            url: "https://example.com".to_string(),
            customization: QrCustomization::default(),
        };
        assert!(request.validate(2048).is_ok());
    }

    #[test]
    fn test_qr_request_validation_empty_url() {
        let request = QrRequest {
            url: "".to_string(),
            customization: QrCustomization::default(),
        };
        assert!(request.validate(2048).is_err());
    }

    #[test]
    fn test_qr_request_validation_url_too_long() {
        let request = QrRequest {
            url: "a".repeat(3000),
            customization: QrCustomization::default(),
        };
        assert!(request.validate(2048).is_err());
    }

    #[test]
    fn test_qr_request_validation_suspicious_url() {
        let request = QrRequest {
            url: "javascript:alert('xss')".to_string(),
            customization: QrCustomization::default(),
        };
        assert!(request.validate(2048).is_err());
    }

    #[test]
    fn test_qr_request_validation_plain_text() {
        let request = QrRequest {
            url: "Hello, World!".to_string(),
            customization: QrCustomization::default(),
        };
        assert!(request.validate(2048).is_ok());
    }

    #[test]
    fn test_qr_customization_validation_success() {
        let customization = QrCustomization::default();
        assert!(customization.validate().is_ok());
    }

    #[test]
    fn test_qr_customization_validation_large_border() {
        let mut customization = QrCustomization::default();
        customization.border_width = 100;
        assert!(customization.validate().is_err());
    }
}