use crate::{
    config::Config,
    errors::ApiError,
    models::v2::{QrCustomization, OutputFormat},
};
use qrcode::QrCode;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use image::Rgba;
use std::io::Cursor;

pub struct QrServiceV2 {
    config: Config,
}

impl QrServiceV2 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn generate_qr_code_v2(
        &self,
        url: &str,
        customization: &QrCustomization,
    ) -> Result<Vec<u8>, ApiError> {
        // Validate inputs
        self.validate_input(url)?;
        customization.validate()?;

        // Create QR code with specified error correction
        let qr_code = QrCode::with_error_correction_level(
            url,
            customization.error_correction.to_qrcode_ecc(),
        )
        .map_err(|e| ApiError::GenerationError(format!("Failed to generate QR code: {}", e)))?;

        // Generate the image based on format
        match customization.format {
            OutputFormat::Png => self.generate_png(&qr_code, customization),
            OutputFormat::Svg => self.generate_svg(&qr_code, customization),
            OutputFormat::Jpeg => self.generate_jpeg(&qr_code, customization),
        }
    }

    pub fn generate_qr_base64_v2(
        &self,
        url: &str,
        customization: &QrCustomization,
    ) -> Result<String, ApiError> {
        let image_data = self.generate_qr_code_v2(url, customization)?;
        let base64_data = BASE64.encode(&image_data);
        
        let data_url = format!(
            "data:{};base64,{}",
            customization.format.content_type(),
            base64_data
        );

        Ok(data_url)
    }

    fn validate_input(&self, url: &str) -> Result<(), ApiError> {
        if url.trim().is_empty() {
            return Err(ApiError::ValidationError("URL cannot be empty".to_string()));
        }

        if url.len() > self.config.max_url_length {
            return Err(ApiError::ValidationError(format!(
                "URL too long (max {} characters)",
                self.config.max_url_length
            )));
        }

        Ok(())
    }

    fn generate_png(
        &self,
        qr_code: &QrCode,
        customization: &QrCustomization,
    ) -> Result<Vec<u8>, ApiError> {
        let size = customization.size.to_pixels();
        
        // Get colors as RGB
        let (fg_r, fg_g, fg_b) = customization.colors.foreground_rgb()
            .map_err(|e| ApiError::ValidationError(e))?;
        let (bg_r, bg_g, bg_b) = customization.colors.background_rgb()
            .map_err(|e| ApiError::ValidationError(e))?;

        // Create RGBA image for better color control
        let image = qr_code.render::<Rgba<u8>>()
            .min_dimensions(size, size)
            .dark_color(Rgba([fg_r, fg_g, fg_b, 255]))
            .light_color(Rgba([bg_r, bg_g, bg_b, 255]))
            .quiet_zone(true) // This adds border
            .build();

        // Convert to PNG bytes
        let mut png_bytes = Vec::new();
        image.write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png)
            .map_err(|e| ApiError::GenerationError(format!("Failed to encode PNG: {}", e)))?;

        Ok(png_bytes)
    }

    fn generate_svg(
        &self,
        qr_code: &QrCode,
        customization: &QrCustomization,
    ) -> Result<Vec<u8>, ApiError> {
        // For now, generate PNG and convert to bytes
        // TODO: Implement proper SVG generation
        let png_data = self.generate_png(qr_code, customization)?;
        
        // Create a simple SVG wrapper for the PNG (temporary solution)
        let size = customization.size.to_pixels();
        let base64_png = BASE64.encode(&png_data);
        let svg_content = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">
  <image href="data:image/png;base64,{}" width="{}" height="{}"/>
</svg>"#,
            size, size, base64_png, size, size
        );

        Ok(svg_content.into_bytes())
    }

    fn generate_jpeg(
        &self,
        qr_code: &QrCode,
        customization: &QrCustomization,
    ) -> Result<Vec<u8>, ApiError> {
        let size = customization.size.to_pixels();
        
        // Get colors as RGB
        let (fg_r, fg_g, fg_b) = customization.colors.foreground_rgb()
            .map_err(|e| ApiError::ValidationError(e))?;
        let (bg_r, bg_g, bg_b) = customization.colors.background_rgb()
            .map_err(|e| ApiError::ValidationError(e))?;

        // Create RGB image (JPEG doesn't support transparency)
        let image = qr_code.render::<image::Rgb<u8>>()
            .min_dimensions(size, size)
            .dark_color(image::Rgb([fg_r, fg_g, fg_b]))
            .light_color(image::Rgb([bg_r, bg_g, bg_b]))
            .quiet_zone(true)
            .build();

        // Convert to JPEG bytes
        let mut jpeg_bytes = Vec::new();
        image.write_to(&mut Cursor::new(&mut jpeg_bytes), image::ImageFormat::Jpeg)
            .map_err(|e| ApiError::GenerationError(format!("Failed to encode JPEG: {}", e)))?;

        Ok(jpeg_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::models::v2::{QrSize, ErrorCorrectionLevel, QrColors};

    fn create_test_config() -> Config {
        Config {
            host: "0.0.0.0".to_string(),
            port: 3000,
            log_level: "info".to_string(),
            max_url_length: 2048,
        }
    }

    fn create_test_customization() -> QrCustomization {
        QrCustomization {
            size: QrSize::Medium,
            error_correction: ErrorCorrectionLevel::M,
            colors: QrColors::default(),
            border_width: 4,
            format: OutputFormat::Png,
        }
    }

    #[test]
    fn test_generate_qr_code_v2_success() {
        let config = create_test_config();
        let service = QrServiceV2::new(config);
        let customization = create_test_customization();
        
        let result = service.generate_qr_code_v2("https://example.com", &customization);
        assert!(result.is_ok());
        
        let png_data = result.unwrap();
        assert!(!png_data.is_empty());
        // PNG files start with specific magic bytes
        assert_eq!(&png_data[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_generate_qr_base64_v2_success() {
        let config = create_test_config();
        let service = QrServiceV2::new(config);
        let customization = create_test_customization();
        
        let result = service.generate_qr_base64_v2("https://example.com", &customization);
        assert!(result.is_ok());
        
        let data_url = result.unwrap();
        assert!(data_url.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn test_generate_svg_format() {
        let config = create_test_config();
        let service = QrServiceV2::new(config);
        let mut customization = create_test_customization();
        customization.format = OutputFormat::Svg;
        
        let result = service.generate_qr_code_v2("https://example.com", &customization);
        assert!(result.is_ok());
        
        let svg_data = result.unwrap();
        let svg_string = String::from_utf8(svg_data).unwrap();
        assert!(svg_string.contains("<svg"));
        assert!(svg_string.contains("</svg>"));
    }

    #[test]
    fn test_custom_colors() {
        let config = create_test_config();
        let service = QrServiceV2::new(config);
        let mut customization = create_test_customization();
        customization.colors = QrColors::new("#FF0000".to_string(), "#00FF00".to_string()).unwrap();
        
        let result = service.generate_qr_code_v2("https://example.com", &customization);
        assert!(result.is_ok());
    }

    #[test]
    fn test_different_sizes() {
        let config = create_test_config();
        let service = QrServiceV2::new(config);
        
        for size in [QrSize::Small, QrSize::Medium, QrSize::Large] {
            let mut customization = create_test_customization();
            customization.size = size;
            
            let result = service.generate_qr_code_v2("https://example.com", &customization);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_error_correction_levels() {
        let config = create_test_config();
        let service = QrServiceV2::new(config);
        
        for ec_level in [
            ErrorCorrectionLevel::L,
            ErrorCorrectionLevel::M,
            ErrorCorrectionLevel::Q,
            ErrorCorrectionLevel::H,
        ] {
            let mut customization = create_test_customization();
            customization.error_correction = ec_level;
            
            let result = service.generate_qr_code_v2("https://example.com", &customization);
            assert!(result.is_ok());
        }
    }
}