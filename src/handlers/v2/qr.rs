use axum::{
    extract::{Query, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
    Json,
};

use crate::{
    errors::ApiError,
    models::v2::{QrRequestV2, QrResponseV2, QrCustomization},
    services::QrServiceV2,
    state::AppState,
};

/// Generate QR code with customization options (JSON response)
/// POST /v2/generate
pub async fn generate_qr_json_v2(
    State(app_state): State<AppState>,
    Json(request): Json<QrRequestV2>,
) -> Result<Json<QrResponseV2>, ApiError> {
    // Validate request
    request.validate(app_state.config.max_url_length)?;

    // Create QR service
    let qr_service = QrServiceV2::new((*app_state.config).clone());
    
    // Generate base64 QR code
    let base64_qr = qr_service.generate_qr_base64_v2(&request.url, &request.customization)?;

    // Create response
    let response = QrResponseV2::new(base64_qr, &request.customization);

    Ok(Json(response))
}

/// Generate QR code with customization options (direct image download)
/// POST /v2/image
pub async fn generate_qr_image_v2(
    State(app_state): State<AppState>,
    Json(request): Json<QrRequestV2>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate request
    request.validate(app_state.config.max_url_length)?;

    // Create QR service
    let qr_service = QrServiceV2::new((*app_state.config).clone());
    
    // Generate image data
    let image_data = qr_service.generate_qr_code_v2(&request.url, &request.customization)?;

    // Set appropriate headers
    let mut headers = HeaderMap::new();
    headers.insert(
        "content-type",
        HeaderValue::from_static(request.customization.format.content_type()),
    );
    
    let filename = format!(
        "qrcode-{}x{}.{}",
        request.customization.size.to_pixels(),
        request.customization.size.to_pixels(),
        request.customization.format.file_extension()
    );
    
    headers.insert(
        "content-disposition",
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename))
            .map_err(|e| ApiError::GenerationError(format!("Invalid filename: {}", e)))?,
    );

    Ok((headers, image_data))
}

/// Generate QR code with query parameters (GET endpoint for backward compatibility)
/// GET /v2/generate?url=<url>&size=<size>&format=<format>&...
pub async fn generate_qr_query_v2(
    State(app_state): State<AppState>,
    Query(params): Query<QrQueryParams>,
) -> Result<Json<QrResponseV2>, ApiError> {
    // Extract URL first to avoid partial move
    let url = params.url.clone();
    let customization = params.into_customization()?;
    
    // Convert query params to request
    let request = QrRequestV2 {
        url,
        customization,
    };

    // Validate request
    request.validate(app_state.config.max_url_length)?;

    // Create QR service
    let qr_service = QrServiceV2::new((*app_state.config).clone());
    
    // Generate base64 QR code
    let base64_qr = qr_service.generate_qr_base64_v2(&request.url, &request.customization)?;

    // Create response
    let response = QrResponseV2::new(base64_qr, &request.customization);

    Ok(Json(response))
}

#[derive(serde::Deserialize)]
pub struct QrQueryParams {
    pub url: String,
    
    #[serde(default)]
    pub size: Option<String>,
    
    #[serde(default)]
    pub format: Option<String>,
    
    #[serde(default)]
    pub error_correction: Option<String>,
    
    #[serde(default)]
    pub foreground_color: Option<String>,
    
    #[serde(default)]
    pub background_color: Option<String>,
    
    #[serde(default)]
    pub border_width: Option<u32>,
}

impl QrQueryParams {
    fn into_customization(self) -> Result<QrCustomization, ApiError> {
        use crate::models::v2::{QrSize, ErrorCorrectionLevel, OutputFormat, QrColors};
        
        let mut customization = QrCustomization::default();
        let _url = self.url; // Move url out first

        // Parse size
        if let Some(size_str) = self.size {
            customization.size = match size_str.to_lowercase().as_str() {
                "small" => QrSize::Small,
                "medium" => QrSize::Medium,
                "large" => QrSize::Large,
                _ => {
                    // Try to parse as custom size
                    let pixels: u32 = size_str.parse()
                        .map_err(|_| ApiError::ValidationError(
                            "Size must be 'small', 'medium', 'large', or a number".to_string()
                        ))?;
                    QrSize::Custom(pixels)
                }
            };
        }

        // Parse format
        if let Some(format_str) = self.format {
            customization.format = match format_str.to_lowercase().as_str() {
                "png" => OutputFormat::Png,
                "svg" => OutputFormat::Svg,
                "jpeg" | "jpg" => OutputFormat::Jpeg,
                _ => return Err(ApiError::ValidationError(
                    "Format must be 'png', 'svg', or 'jpeg'".to_string()
                )),
            };
        }

        // Parse error correction
        if let Some(ec_str) = self.error_correction {
            customization.error_correction = match ec_str.to_uppercase().as_str() {
                "L" => ErrorCorrectionLevel::L,
                "M" => ErrorCorrectionLevel::M,
                "Q" => ErrorCorrectionLevel::Q,
                "H" => ErrorCorrectionLevel::H,
                _ => return Err(ApiError::ValidationError(
                    "Error correction must be 'L', 'M', 'Q', or 'H'".to_string()
                )),
            };
        }

        // Parse colors
        if self.foreground_color.is_some() || self.background_color.is_some() {
            let fg = self.foreground_color.unwrap_or_else(|| "#000000".to_string());
            let bg = self.background_color.unwrap_or_else(|| "#FFFFFF".to_string());
            
            customization.colors = QrColors::new(fg, bg)
                .map_err(|e| ApiError::ValidationError(e))?;
        }

        // Parse border width
        if let Some(border) = self.border_width {
            customization.border_width = border;
        }

        // Validate the complete customization
        customization.validate()?;

        Ok(customization)
    }
}