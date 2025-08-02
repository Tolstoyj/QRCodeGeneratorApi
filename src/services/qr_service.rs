use base64::Engine;
use qrcode::QrCode;
use std::io::Cursor;

use crate::errors::AppError;

pub struct QrService;

impl QrService {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_qr_code(&self, data: &str) -> Result<Vec<u8>, AppError> {
        let code = QrCode::new(data)
            .map_err(|e| AppError::QrGeneration(e.to_string()))?;

        let image = code.render::<image::Luma<u8>>().build();
        
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        
        image.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| AppError::ImageEncoding(e.to_string()))?;

        Ok(buffer)
    }

    pub fn generate_base64_qr(&self, data: &str) -> Result<String, AppError> {
        let buffer = self.generate_qr_code(data)?;
        let base64_image = base64::engine::general_purpose::STANDARD.encode(&buffer);
        Ok(format!("data:image/png;base64,{}", base64_image))
    }
}