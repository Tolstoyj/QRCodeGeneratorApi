use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QrSize {
    Small,
    Medium,
    Large,
    Custom(u32),
}

impl QrSize {
    pub fn to_pixels(&self) -> u32 {
        match self {
            QrSize::Small => 150,
            QrSize::Medium => 300,
            QrSize::Large => 600,
            QrSize::Custom(size) => *size,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        match self {
            QrSize::Custom(size) if *size < 50 => {
                Err("Custom size must be at least 50 pixels".to_string())
            }
            QrSize::Custom(size) if *size > 2000 => {
                Err("Custom size cannot exceed 2000 pixels".to_string())
            }
            _ => Ok(()),
        }
    }
}

impl Default for QrSize {
    fn default() -> Self {
        QrSize::Medium
    }
}

impl fmt::Display for QrSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QrSize::Small => write!(f, "small (150px)"),
            QrSize::Medium => write!(f, "medium (300px)"),
            QrSize::Large => write!(f, "large (600px)"),
            QrSize::Custom(size) => write!(f, "custom ({}px)", size),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ErrorCorrectionLevel {
    L, // Low (~7%)
    M, // Medium (~15%) - Default
    Q, // Quartile (~25%)
    H, // High (~30%)
}

impl ErrorCorrectionLevel {
    pub fn to_qrcode_ecc(&self) -> qrcode::EcLevel {
        match self {
            ErrorCorrectionLevel::L => qrcode::EcLevel::L,
            ErrorCorrectionLevel::M => qrcode::EcLevel::M,
            ErrorCorrectionLevel::Q => qrcode::EcLevel::Q,
            ErrorCorrectionLevel::H => qrcode::EcLevel::H,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ErrorCorrectionLevel::L => "Low (~7% recovery)",
            ErrorCorrectionLevel::M => "Medium (~15% recovery)",
            ErrorCorrectionLevel::Q => "Quartile (~25% recovery)",
            ErrorCorrectionLevel::H => "High (~30% recovery)",
        }
    }
}

impl Default for ErrorCorrectionLevel {
    fn default() -> Self {
        ErrorCorrectionLevel::M
    }
}

impl fmt::Display for ErrorCorrectionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} - {}", self, self.description())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Png,
    Svg,
    Jpeg,
}

impl OutputFormat {
    pub fn content_type(&self) -> &'static str {
        match self {
            OutputFormat::Png => "image/png",
            OutputFormat::Svg => "image/svg+xml",
            OutputFormat::Jpeg => "image/jpeg",
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            OutputFormat::Png => "png",
            OutputFormat::Svg => "svg",
            OutputFormat::Jpeg => "jpg",
        }
    }
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Png
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_size_pixels() {
        assert_eq!(QrSize::Small.to_pixels(), 150);
        assert_eq!(QrSize::Medium.to_pixels(), 300);
        assert_eq!(QrSize::Large.to_pixels(), 600);
        assert_eq!(QrSize::Custom(500).to_pixels(), 500);
    }

    #[test]
    fn test_qr_size_validation() {
        assert!(QrSize::Small.validate().is_ok());
        assert!(QrSize::Custom(100).validate().is_ok());
        assert!(QrSize::Custom(30).validate().is_err());
        assert!(QrSize::Custom(3000).validate().is_err());
    }

    #[test]
    fn test_error_correction_mapping() {
        assert!(matches!(
            ErrorCorrectionLevel::L.to_qrcode_ecc(),
            qrcode::EcLevel::L
        ));
        assert!(matches!(
            ErrorCorrectionLevel::M.to_qrcode_ecc(),
            qrcode::EcLevel::M
        ));
    }

    #[test]
    fn test_output_format_properties() {
        assert_eq!(OutputFormat::Png.content_type(), "image/png");
        assert_eq!(OutputFormat::Svg.content_type(), "image/svg+xml");
        assert_eq!(OutputFormat::Png.file_extension(), "png");
        assert_eq!(OutputFormat::Svg.file_extension(), "svg");
    }
}