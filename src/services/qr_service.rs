use crate::config::Config;
use crate::errors::ApiError;
use qrcode::QrCode;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub struct QrService {
    config: Config,
}

impl QrService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn generate_qr_code(&self, url: &str) -> Result<String, ApiError> {
        // Validate URL length
        if url.len() > self.config.max_url_length {
            return Err(ApiError::ValidationError(format!(
                "URL too long (max {} characters)",
                self.config.max_url_length
            )));
        }

        // Validate URL is not empty
        if url.trim().is_empty() {
            return Err(ApiError::ValidationError("URL cannot be empty".to_string()));
        }

        // Generate QR code
        let qr_code = QrCode::new(url)
            .map_err(|e| ApiError::GenerationError(format!("Failed to generate QR code: {}", e)))?;

        // Convert to image
        let image_buffer = qr_code.render::<image::Luma<u8>>().build();

        // Convert to PNG bytes
        let mut png_bytes = Vec::new();
        image_buffer.write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png)
            .map_err(|e| ApiError::GenerationError(format!("Failed to encode PNG: {}", e)))?;

        // Convert to base64
        let base64_string = BASE64.encode(&png_bytes);
        let data_url = format!("data:image/png;base64,{}", base64_string);

        Ok(data_url)
    }

    pub fn generate_qr_image(&self, url: &str) -> Result<Vec<u8>, ApiError> {
        // Validate URL length
        if url.len() > self.config.max_url_length {
            return Err(ApiError::ValidationError(format!(
                "URL too long (max {} characters)",
                self.config.max_url_length
            )));
        }

        // Validate URL is not empty
        if url.trim().is_empty() {
            return Err(ApiError::ValidationError("URL cannot be empty".to_string()));
        }

        // Generate QR code
        let qr_code = QrCode::new(url)
            .map_err(|e| ApiError::GenerationError(format!("Failed to generate QR code: {}", e)))?;

        // Convert to image
        let image_buffer = qr_code.render::<image::Luma<u8>>().build();

        // Convert to PNG bytes
        let mut png_bytes = Vec::new();
        image_buffer.write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png)
            .map_err(|e| ApiError::GenerationError(format!("Failed to encode PNG: {}", e)))?;

        Ok(png_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    fn create_test_config() -> Config {
        Config {
            host: "0.0.0.0".to_string(),
            port: 3000,
            log_level: "info".to_string(),
            max_url_length: 2048,
        }
    }

    #[test]
    fn test_generate_qr_code_success() {
        let config = create_test_config();
        let service = QrService::new(config);
        let result = service.generate_qr_code("https://example.com");
        
        assert!(result.is_ok());
        let data_url = result.unwrap();
        assert!(data_url.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn test_generate_qr_code_empty_url() {
        let config = create_test_config();
        let service = QrService::new(config);
        let result = service.generate_qr_code("");
        
        assert!(result.is_err());
        match result {
            Err(ApiError::ValidationError(msg)) => {
                assert_eq!(msg, "URL cannot be empty");
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_generate_qr_code_url_too_long() {
        let config = create_test_config();
        let service = QrService::new(config);
        let long_url = "a".repeat(3000);
        let result = service.generate_qr_code(&long_url);
        
        assert!(result.is_err());
        match result {
            Err(ApiError::ValidationError(msg)) => {
                assert!(msg.contains("URL too long"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_generate_qr_image_success() {
        let config = create_test_config();
        let service = QrService::new(config);
        let result = service.generate_qr_image("https://example.com");
        
        assert!(result.is_ok());
        let png_bytes = result.unwrap();
        assert!(!png_bytes.is_empty());
        // PNG files start with specific bytes
        assert_eq!(&png_bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_generate_qr_image_empty_url() {
        let config = create_test_config();
        let service = QrService::new(config);
        let result = service.generate_qr_image("");
        
        assert!(result.is_err());
        match result {
            Err(ApiError::ValidationError(msg)) => {
                assert_eq!(msg, "URL cannot be empty");
            }
            _ => panic!("Expected ValidationError"),
        }
    }
}